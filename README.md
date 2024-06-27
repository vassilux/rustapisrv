# rustapisrv

API server in Rust for learning purposes.

This project demonstrates how to create an API server using Rust, following a Clean Architecture structure.

## Project Structure

The project is organized following the principles of Clean Architecture, which separates the application into different layers: Framework, Adapters, and Domain.

my_project/
├── Cargo.toml
├── src/
│ ├── main.rs
│ ├── lib.rs
│ ├── adapters/
│ │ ├── controllers/
│ │ │ ├── v1/
│ │ │ │ └── mod.rs
│ │ │ └── mod.rs
│ │ ├── repositories/
│ │ │ └── mod.rs
│ │ └── mod.rs
│ ├── domain/
│ │ ├── entities/
│ │ │ └── mod.rs
│ │ ├── use_cases/
│ │ │ └── mod.rs
│ │ └── mod.rs
│ └── framework/
│ ├── config/
│ │ └── mod.rs
│ ├── database/
│ │ └── mod.rs
│ ├── web/
│ │ ├── v1/
│ │ │ └── mod.rs
│ │ └── mod.rs
│ └── mod.rs


## Getting Started

To run this project, you will need to have Rust installed on your system. If you don't have it installed, you can get it from [rust-lang.org](https://www.rust-lang.org/).

### Prerequisites

- Rust (latest stable version)
- Oracle Instant Client (if you are planning to use Oracle DB)
- Docker (optional, for running a local database instance)

### Installation

1. Clone the repository:

```sh
git clone https://github.com/yourusername/rustapisrv.git
cd rustapisrv
```

Build the project:

cargo build


Running the Server
The server will start at http://127.0.0.1:8080 by default.

### Endpoints

Create User

POST /v1/user


Request Body:

{
  "name": "Your Name"
}

Response Body: 

{
  "id": 1,
  "name": "Your Name"
}


Get User

GET /v1/user/{id}


Response Body:

{
  "id": 1,
  "name": "Your Name"
}

### Contributing 

Feel free to fork this project and submit pull requests. Any contributions are welcome!