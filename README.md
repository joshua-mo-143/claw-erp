## Claw: A Rust-based CLI ERP
Claw aims to be a blazing-fast command-line Enterprise Resource Planning (ERP for short) application, written in Rust.

What is Enterprise Resource Planning? Essentially, it's the integrated management of main business processes. This means that the application will (eventually be able to) handle invoicing and sales, accounts payable and accounts receivable as well as watching stock levels if applicable, so there will be one centralised application for daily business operations. 

## Feature list (TODO)
- Invoicing
- Customer list
- Handle payments (Stripe integrations)
- Store invoices
- SQLite integrations, with ability to use other forms of databases


## Technical design justifications
Having used business solutions like SAP Business One, I have found ERP business solutions typically to be very difficult to use and have very cluttered UI. This has informed my technical design choices when creating this app; instead of having a cluttered GUI, the user will simply be able to type commands to bring up what they need (so if you're in the app, you could just type "invoice new"). Typically, this could be solved by using things like keyboard shortcuts, but I've found that less technical users tend to not know how to set keyboard shortcuts.

With this, I've decided to 