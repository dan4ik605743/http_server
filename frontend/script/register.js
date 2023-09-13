
        // Обработчик события отправки формы
        document.getElementById("data-form").addEventListener("submit", function (event) {
            event.preventDefault(); // Предотвращаем отправку формы по умолчанию

            // Получение данных из формы
            const username = document.getElementById("username").value;
            const password = document.getElementById("password").value;

            // Создание объекта с данными для отправки на сервер
            const data = {
                username: username,
                password: password
            };

            // Отправка данных на сервер с использованием AJAX
            const xhr = new XMLHttpRequest();
            xhr.open("POST", "/register", true);
            xhr.setRequestHeader("Content-Type", "application/json;charset=UTF-8");
            xhr.onreadystatechange = function () {
                if (xhr.readyState === XMLHttpRequest.DONE) {
                    if (xhr.status === 200) {
                        console.log("Данные успешно отправлены на сервер!");

                        // Добавляем уведомление о успешной отправке данных
                        alert("Вы успешно зарегистрированны!");

                        // После успешной регистрации перенаправляем на страницу "/"
                        window.location.href = "/";

                    } else if (xhr.status === 409) {
                        console.log("Пользователь с таким именем уже существует.");
                        alert("Пользователь с таким именем существует.");
                    } else {
                        alert("Произошла ошибка в регистрации!");
                        console.log("Произошла ошибка при отправке данных на сервер.");
                        console.log("Статус ошибки:", xhr.status);
                    }
                }
            };
            xhr.send(JSON.stringify(data));

            // Очистка формы после отправки данных
            document.getElementById("data-form").reset();
        });

        // Функция для перехода на страницу "/"
        function goBack() {
            window.location.href = "/";
        }