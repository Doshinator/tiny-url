services:
  db:
    image: postgres:16
    container_name: tiny-url-postgres
    environment:
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: password
      POSTGRES_DB: tiny_url
    ports:
      - "5432:5432"