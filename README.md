# 🗳️ Rust CLI Poll Voting System

A fully-featured command-line poll voting application built in **Rust** with persistent data storage in **PostgreSQL** — containerized using **Docker** for easy deployment and testing.

---

## 📦 Features

✅ User registration and login  
✅ Create polls with up to 4 choices  
✅ View all polls or your own created polls  
✅ Vote only once per poll  
✅ Prevent duplicate voting (tracked by user ID)  
✅ Show vote results with live percentages and visual bars  
✅ Color-coded terminal output with `colored` crate  
✅ Fully Dockerized: PostgreSQL + CLI app

---

## 🧠 System Design Overview

### 🧩 Architecture

+-------------+ Docker Compose +---------------+
| User CLI | <--------------------> | PostgreSQL |
| (Rust App) | | (Persistent DB)|
+-------------+ +---------------+


- Rust app runs in an isolated container
- Connects to a PostgreSQL container via `DATABASE_URL`
- Uses SQLx for DB queries
- CLI interacts with user for creating and voting on polls

---

## 🗃️ Database Schema

### `users` table

```sql
id UUID PRIMARY KEY,
username TEXT UNIQUE NOT NULL,
user_creation_time TIMESTAMP NOT NULL,
voted_polls UUID[] NOT NULL

### `polls` table
id UUID PRIMARY KEY,
question TEXT NOT NULL,
choices TEXT[] NOT NULL,
vote_counts INTEGER[] NOT NULL,
creation_time TIMESTAMP NOT NULL,
deadline TIMESTAMP,
created_by UUID REFERENCES users(id)

🚀 Getting Started (with Docker)

🔧 Prerequisites
Docker installed on your system
docker compose available (Docker Desktop includes it)

📁 Step 1: Clone the repo
git clone this repo

🛠 Step 2: Build & Start the project
docker compose up --build

✅ This will:

Build the Rust app from source
Start a PostgreSQL database
Connect both in a shared network
🖥️ Step 3: Run the CLI in your own terminal
You can run the app directly inside the container using:

🧪 Usage Flow

On startup, user is asked if they have an existing username
If no → register a new user
Menu:
1- Create a poll
2- View all polls
3- Vote on a poll
4- View your polls
5- View polls you’ve voted on
6- Exit

🧱

🦀 Rust
🐘 PostgreSQL
🐳 Docker & Compose
🧵 sqlx for async DB
🎨 colored for CLI styling

💡 Ideas for Future Improvements

Add password-based authentication
Export poll results to JSON/CSV
Add web API using Axum or Actix
Frontend: build a React/Vue dashboard
Time-based poll expiration and automatic closure

📄 License

MIT © 2024 Ghaleb