# 🗳️ RustPoll — A Rust CLI Polling App with PostgreSQL

RustPoll is a simple command-line voting app built in Rust, using PostgreSQL as a backend. It allows users to register via the terminal, automatically checks for existing accounts, and stores everything in a real database.

---

## ✅ Features So Far

- Terminal-based interface (`cargo run`)
- User creation with input validation
- Check if a user already exists in the database
- PostgreSQL integration using `sqlx`
- All user data stored and queried from the database
- pgAdmin can be used to view data, but all actions are handled from the terminal

---

## 🧱 Project Structure

src/
├── main.rs # Entry point and CLI menu
├── db.rs # Connects to PostgreSQL
├── user.rs # Handles user creation and lookup
├── models.rs # Contains the User struct


---

## 🛢️ Database Setup

**Database name:** `rust_poll`

**Table created:**

```sql
CREATE TABLE users (
    id UUID PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    user_creation_time TIMESTAMP NOT NULL,
    voted_polls UUID[] NOT NULL DEFAULT '{}'
);

🔧 Technologies Used

🦀 Rust (async)
🐘 PostgreSQL (installed locally)
📦 SQLx (for database queries)
🖥️ pgAdmin 4 (for DB inspection, not required)
🧭 Next Goals

 Poll creation and storage
 Voting system with vote tracking
 CLI menu to navigate options
 Optional: SQLx migrations and Docker support