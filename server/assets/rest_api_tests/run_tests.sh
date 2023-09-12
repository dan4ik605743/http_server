#!/bin/bash

# Выполняем тесты из get.py
pytest ./server/assets/rest_api_tests/get.py

# Проверяем статус выполнения pytest get.py
if [ $? -ne 0 ]; then
  echo "Ошибка при выполнении тестов в get.py"
  exit 1
fi

# Выполняем тесты из post.py
pytest ./server/assets/rest_api_tests/post.py

# Проверяем статус выполнения pytest post.py
if [ $? -ne 0 ]; then
  echo "Ошибка при выполнении тестов в post.py"
  exit 1
fi

echo "Все тесты успешно выполнены"

