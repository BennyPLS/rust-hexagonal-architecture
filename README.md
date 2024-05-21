# Rust Hexagonal Architecture

## Architectural Patters

- [ ] Hexagonal Architecture
- [ ] SOLID Principles (Pragmatically)
    - [ ] Dependency Inversion & Injection
    - [ ] Interface Segregation
    - [ ] Single Responsibility
    - [ ] Event Driven Architecture / Design
- [ ] Event Driven Architecture / Design

## Integrations

- [ ] Kafka / RabbitMQ
- [ ] Docker
- [ ] SQLite (No ORM)
- [ ] MySQL (ORM)
- [ ] NoSQL
- [ ] NoSQL (ORM)

## Features

### User

#### Structure

````rust
pub struct User {
    id: String,
    name: String,
    password: String,
    email: String,
}
````

##### Value Objects Validation

- [ ] ID
- [ ] Name
- [ ] Password
- [ ] Email

##### Features

- [ ] Password Hashing
- [ ] Use and ID Standard (UUID or Alternatives)

#### CRUD

##### Endpoints (Rocket)

- [ ] Create / Register
- [ ] Read
    - [ ] Read One
    - [ ] Read All
    - [ ] Criteria
- [ ] Update
- [ ] Delete

#### Database Representations

##### SQLite

- [X] Create / Register
- [X] Read
    - [X] Read One
    - [X] Read All
    - [ ] Criteria
- [X] Update
- [X] Delete

##### Undefined (ORM)

- [ ] Create / Register
- [ ] Read
    - [ ] Read One
    - [ ] Read All
    - [ ] Criteria
- [ ] Update
- [ ] Delete
