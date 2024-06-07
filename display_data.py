import os
import psycopg2
from dotenv import load_dotenv
from pprint import pprint

# Load environment variables from .env file
load_dotenv()

# Get the database connection string from the .env file
DATABASE_URL = os.getenv("DATABASE_URL")

# Check if DATABASE_URL is loaded correctly
if DATABASE_URL is None:
    raise ValueError("DATABASE_URL not found in .env file")


def fetch_todo_list():
    try:
        # Connect to the PostgreSQL database
        conn = psycopg2.connect(DATABASE_URL)
        cursor = conn.cursor()

        # Execute the query to fetch all data from the todo_list table
        cursor.execute("SELECT * FROM todo_items")

        # Fetch all rows from the executed query
        rows = cursor.fetchall()

        # Fetch column names from the cursor description
        column_names = [desc[0] for desc in cursor.description]

        # Close the cursor and the connection
        cursor.close()
        conn.close()

        # Format the rows as a list of dictionaries
        todo_list = [dict(zip(column_names, row)) for row in rows]

        return todo_list
    except Exception as e:
        print(f"Error fetching data: {e}")
        return []


def main():
    todo_list = fetch_todo_list()

    # Pretty print the todo_list similar to Rust's {:#?}
    pprint(todo_list, width=120)


if __name__ == "__main__":
    main()
