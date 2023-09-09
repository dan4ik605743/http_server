// Обработчик события отправки формы
document.getElementById("login-form").addEventListener("submit", function (event) {
    event.preventDefault(); // Предотвращаем отправку формы по умолчанию

    // Получение данных из формы
    const username = document.getElementById("username").value;
    const password = document.getElementById("password").value;

    // Создание объекта с данными для отправки на сервер
    const data = {
        username: username,
        password: password
    };

    // Отправка данных на сервер с использованием Fetch API
    fetch("/login", {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify(data)
    })
        .then(response => response.json()) // Преобразуем ответ в JSON
        .then(responseData => {
            if (responseData.message === "OK") {
                // Вход выполнен успешно, переадресация на /user с данными
                window.location.href = `/user?username=${responseData.username}`;
            } else if (responseData.error === "Not Found") {
                // Пользователь не найден
                alert("Пользователь не найден.");
            } else if (responseData.error === "Unauthorized") {
                // Неверный пароль пользователя
                alert("Пароль пользователя неверен.");
            } else {
                // Обработка других ошибок
                alert("Произошла ошибка при входе.");
            }
        })
        .catch(error => {
            console.error("Произошла ошибка при выполнении запроса:", error);
            alert("Произошла ошибка при входе.");
        });

    // Очистка формы после отправки данных
    document.getElementById("login-form").reset();
});

// Функция для перехода на страницу "/"
function goBack() {
    window.location.href = "/";
}