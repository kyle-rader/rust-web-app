# This file should contain all the record creation needed to seed the database with its default values.
# The data can then be loaded with the bin/rails db:seed command (or created alongside the database with db:setup).

accounts = Account.create([
    { email: "kylewrader@gmail.com", password: "astrongpassword", status: 2, alias: 'mcyamaha' },
    { email: "user1@gmail.com", password: "astrongpassword", status: 2, alias: 'user1' },
    { email: "user2@gmail.com", password: "astrongpassword", status: 2, alias: 'user2' },
    { email: "user3@gmail.com", password: "astrongpassword", status: 2, alias: 'user3' },
])