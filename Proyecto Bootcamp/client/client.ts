import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import type { SolanaTodoApp } from "../target/types/solana_todo_app";

// Configure the client to use the local cluster
anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.SolanaTodoApp as anchor.Program<SolanaTodoApp>;

// Configura el cliente con el proveedor y wallet conectados en Solana Playground
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = program;
const authority = provider.wallet.publicKey;

async function main() {
  console.log("Wallet / Authority:", authority.toBase58());
  console.log("Program ID:", program.programId.toBase58());

  // 1. Derivar la PDA padre: UserProfile
  const [userProfilePda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("user-profile"), authority.toBuffer()],
    program.programId
  );
  console.log("UserProfile PDA:", userProfilePda.toBase58());

  // 2. Inicializar el perfil en Devnet (si aún no existe)
  const existingProfile = await provider.connection.getAccountInfo(userProfilePda);
  if (!existingProfile) {
    const txInit = await program.methods
      .initializeProfile()
      .accounts({
        userProfile: userProfilePda,
        authority: authority,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Perfil creado! Firma (tx):", txInit);
  } else {
    console.log("El perfil ya existía en Devnet.");
  }

  // 3. Leer el perfil para obtener task_count actual
  const profileAccount = await program.account.userProfile.fetch(userProfilePda);
  const taskCount = BigInt(profileAccount.taskCount.toString());

  // Convertir taskCount (u64) a 8 bytes little-endian (igual que to_le_bytes() en Rust)
  const taskIdBuffer = Buffer.alloc(8);
  taskIdBuffer.writeBigUInt64LE(taskCount);

  // 4. Derivar la PDA hija: TaskAccount
  const [taskAccountPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("task"), authority.toBuffer(), taskIdBuffer],
    program.programId
  );

  // 5. Crear una tarea de prueba en Devnet
  const txTask = await program.methods
    .createTask("Mi primera tarea en Solana Devnet")
    .accounts({
      userProfile: userProfilePda,
      taskAccount: taskAccountPda,
      authority: authority,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();

  console.log("Tarea creada! Firma (tx):", txTask);
  console.log("TaskAccount PDA:", taskAccountPda.toBase58());
}

main().catch((err) => {
  console.error("Error:", err);
});
