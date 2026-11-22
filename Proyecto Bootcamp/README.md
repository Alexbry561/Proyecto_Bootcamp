## Solana To-Do dApp (Devnet)

## Ruta elegida

- Ruta: Proyecto Propio

## Red: Solana Devnet (https://api.devnet.solana.com)

## Propósito

- Aplicación descentralizada (dApp) de lista de tareas (To-Do) desplegada en Solana Devnet, compuesta por un programa en Rust/Anchor y una interfaz web (index.html) que permite conectar una wallet de Solflare, inicializar un perfil de usuario mediante un PDA (UserProfile PDA) y crear tareas en la blockchain vinculadas al perfil incrementando task_count.

## Estructura del proyecto

- Programa (Rust / Anchor): programs/ (incluye la lógica en lib.rs con initialize_profile, create_task y la lógica de estado de tareas).

- Frontend: index.html (interfaz de conexión con Solflare, gestión de perfil PDA y creación de tareas en Devnet).

- Cliente e IDL: client/ y target/ (archivos de cliente e IDL generados/exportados correspondientes a la versión desplegada).

- Configuración del workspace: Anchor.toml, Cargo.toml, package.json, migrations/ y tests/.

## Configuración

- Program ID (Devnet): 3So3vydxxPhLpXvRctidue72oe65fgsLXyk25watyTCU

- Wallet utilizada en pruebas: CYNbpr8hK9c1evUBwdErjYsyP9nZZ5Cq1p1vfss8RbXJ

- UserProfile PDA (Devnet): 4wUCo97Lyip2fokFnzX1VrcNNx91FVD88KWM4dPDDTmD

- Wallet extension requerida: Solflare configurada en Devnet con saldo de prueba (SOL de Devnet).

## Pasos de ejecución (en local)

- Abre la carpeta del proyecto en Visual Studio Code (o tu editor de preferencia).

- Inicia un servidor local para servir index.html (por ejemplo, con Live Server en http://127.0.0.1:5500/index.html).

- Asegúrate de que tu wallet Solflare esté desbloqueada y cambiada a la red Devnet.

- Accede a http://127.0.0.1:5500/index.html en tu navegador.

## Aporte propio

- Integré la interfaz web (index.html) con la wallet de Solflare en Solana Devnet, conectando el programa desplegado (3So3vydxxPhLpXvRctidue72oe65fgsLXyk25watyTCU) con la inicialización del perfil PDA (UserProfile PDA) y la creación de tareas (create_task) incrementando task_count.

- Exporté y organicé la estructura del proyecto junto con el frontend, las evidencias y la documentación para dejarlo listo en el repositorio de entrega.

## Cómo probarlo

- Conectar wallet: Haz clic en Conectar Solflare (CYNbpr8hK9c1evUBwdErjYsyP9nZZ5Cq1p1vfss8RbXJ) y verifica que muestre la red Devnet y tu saldo disponible.

- Verificar o inicializar perfil: Haz clic en Actualizar Estado del Perfil (o en Inicializar Perfil (initialize_profile) si el perfil aún no existe) y confirma que aparezca Estado: Inicializado ✓ junto con UserProfile PDA (4wUCo97Lyip2fokFnzX1VrcNNx91FVD88KWM4dPDDTmD) y Tareas registradas (task_count).

- Crear una tarea: Escribe un título de tarea (máximo 64 caracteres) en 3. Crear Tarea (create_task), haz clic en Crear Tarea en Devnet, aprueba la firma en Solflare y comprueba que task_count aumente (por ejemplo, de 1 a 2) y se muestre la Task PDA y la firma (tx).

## Evidencias y transacciones verificables

- Última tarea creada en pruebas: Task PDA iniciada en DWVURUB...dPSP31Y y firma (tx) visible en la interfaz (SueefPk1rPhGcP13hbFn4HCtsybohYUBmJBfnWYMJSrUB5HEWRKv6UEXewY7np3cF4VYHEynHcUA9iQSPf2h2tya).

- Explorador (Devnet): Consulta las firmas y cuentas en Solana Explorer agregando ?cluster=devnet.

- Capturas incluidas: Screenshot 2026-11-21 221227.png (y capturas adicionales de la ejecución en Devnet).

## Identificación

- Nombre para el certificado: Bryant Alexander Escobar Loera

- Correo de registro: bryantescobar767@gmail.com