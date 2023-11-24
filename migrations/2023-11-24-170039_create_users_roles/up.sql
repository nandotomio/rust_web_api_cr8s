CREATE TABLE users_roles (
  id SERIAL PRIMARY KEY,
  user_id INTEGER REFERENCES users(id) NOT NULL,
  role_id INTEGER REFERENCES roles(id) NOT NULL
);