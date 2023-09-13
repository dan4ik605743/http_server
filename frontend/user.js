// Получение данных пользователя из URL-адреса
const urlParams = new URLSearchParams(window.location.search);
const username = urlParams.get("username");

// Отображение имени пользователя
document.getElementById("username").textContent = `Username: ${username}`;