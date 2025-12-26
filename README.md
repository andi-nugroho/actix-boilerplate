# 📝💰 Notes & Money Tracker API

Sebuah aplikasi REST API yang keren banget buat nyatet notes dan track pengeluaran uang elu! Built with Rust menggunakan Actix Web dan PostgreSQL.

## ✨ Features

### 📝 Notes Management
- Create, read, update, delete notes
- Search notes by title/content
- UUID-based IDs (more secure!)
- Timestamps tracking

### 💰 Money Transaction Tracking  
- Track income & expenses
- Categories (Food, Transportation, Entertainment, etc.)
- Real-time balance calculation
- Category-wise summaries
- Search transactions
- Filter by transaction type

## 🏗️ Architecture

Aplikasi ini dibuat menggunakan **SOLID principles** dengan clean architecture:

```
src/
├── config/          # Configuration management
├── models/          # Data structures & domain models
├── repositories/    # Database access layer
├── services/        # Business logic layer
├── controllers/     # HTTP handlers
├── utils/           # Helper functions & migrations
├── lib.rs           # Module declarations
└── main.rs          # Application entry point
```

## 🚀 Getting Started

### Prerequisites
- Rust 1.70+ 
- PostgreSQL 13+

### Installation

1. Clone this repo
```bash
git clone <your-repo>
cd crud-test
```

2. Set up environment variables
```bash
# Copy and edit the environment file
DATABASE_URL=postgresql://username:password@localhost/crud_test
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
RUST_LOG=info
```

3. Create database
```bash
createdb crud_test
```

4. Run the application
```bash
cargo run
```

The server will start at `http://127.0.0.1:8080` 🎉

## 📖 API Documentation

### Notes Endpoints

#### Get All Notes
```
GET /api/notes
GET /api/notes?q=search_term
```

#### Create Note
```
POST /api/notes
Content-Type: application/json

{
  "title": "My Note",
  "content": "This is the content"
}
```

#### Get Note by ID
```
GET /api/notes/{note_id}
```

#### Update Note
```
PUT /api/notes/{note_id}
Content-Type: application/json

{
  "title": "Updated Title",
  "content": "Updated content"
}
```

#### Delete Note
```
DELETE /api/notes/{note_id}
```

### Money Transaction Endpoints

#### Get All Transactions
```
GET /api/transactions
GET /api/transactions?q=search_term
GET /api/transactions?transaction_type=income
GET /api/transactions?transaction_type=expense
```

#### Create Transaction
```
POST /api/transactions
Content-Type: application/json

{
  "title": "Lunch at McD",
  "description": "Burger + fries",
  "amount": 45.50,
  "transaction_type": "expense",
  "category": "food"
}
```

#### Get Transaction by ID
```
GET /api/transactions/{transaction_id}
```

#### Update Transaction
```
PUT /api/transactions/{transaction_id}
Content-Type: application/json

{
  "title": "Updated transaction",
  "amount": 50.00
}
```

#### Delete Transaction
```
DELETE /api/transactions/{transaction_id}
```

### Money Summary Endpoints

#### Get Balance Info
```
GET /api/money/balance

Response:
{
  "balance": 1500.50,
  "total_income": 2000.00,
  "total_expense": 499.50
}
```

#### Get Category Summary
```
GET /api/money/summary
GET /api/money/summary?transaction_type=expense

Response:
[
  {
    "category": "food",
    "total_amount": 150.75,
    "transaction_count": 8
  }
]
```

## 🗂️ Transaction Categories

- `food` - Makanan & minuman
- `transportation` - Transport (bensin, ojol, dll)
- `entertainment` - Hiburan (nonton, games, dll)
- `shopping` - Belanja (baju, gadget, dll)
- `bills` - Tagihan (listrik, internet, dll)
- `salary` - Gaji & income tetap
- `investment` - Investasi & tabungan
- `other` - Lain-lain

## 🛠️ Tech Stack

- **Rust** - Programming language
- **Actix Web** - Web framework
- **SQLX** - Async SQL toolkit & ORM
- **PostgreSQL** - Database
- **UUID** - Unique identifiers
- **Chrono** - Date & time handling
- **Serde** - Serialization framework

## 🎯 SOLID Principles Implementation

- **S**ingle Responsibility: Each module has one clear purpose
- **O**pen/Closed: Easy to extend with new features
- **L**iskov Substitution: Proper abstraction layers
- **I**nterface Segregation: Clean service interfaces
- **D**ependency Inversion: Repository pattern for data access

## 🚧 Future Enhancements

- [ ] User authentication & authorization
- [ ] Monthly/yearly financial reports
- [ ] Export data to CSV/Excel
- [ ] File attachments for notes
- [ ] Recurring transactions
- [ ] Budget planning & alerts
- [ ] GraphQL API
- [ ] Frontend dashboard

## 🤝 Contributing

Feel free to contribute! Open issues, submit PRs, whatever you want!

## 📄 License

MIT License - Do whatever you want with this code! 

---

Built with ❤️ and lots of ☕ by a developer who loves clean code! 