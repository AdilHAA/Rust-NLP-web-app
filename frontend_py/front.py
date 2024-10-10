import streamlit as st
import requests

def get_answer(text):
    # Not implemented
    print(text)

def parse(link):
    # Not implemented
    print(link)
    return 'До связи'

st.set_page_config(
    page_title='Классификатор новостей',
    page_icon=':news:',
    layout='wide'
)

# Sidebar
st.sidebar.title("Инфармация")
st.sidebar.info(
    "Проект для програменг технологии"
)

# Main Page
st.title("Классификатор новостей")

method = st.radio(
    "Выберите режим:",
    ["Ввести текст новости", "Ввести ссылку на news.mail.ru"],
    index=0,
)

if method == 'Ввести текст новости':
    with st.form('my_form'):
        text = st.text_area('Введите новость:', '')
        submitted = st.form_submit_button('Submit')
        if submitted:
            st.write("Отправка запроса к бэкенду...")
            try:
                response = requests.post("http://127.0.0.1:8080/analyze", json={"text": text})  # Исправлено поле JSON
                st.write(f"Статус ответа: {response.status_code}")

                if response.status_code == 200:
                    result = response.json()
                    st.success(f"Результат: {result}")
                else:
                    st.error("Ошибка при обращении к Rust API")
            except requests.exceptions.ConnectionError:
                st.error("Не удалось подключиться к Rust API. Убедитесь, что бэкенд запущен.")
else:
    with st.form('link_form'):
        link = st.text_input('Введите ссылку на новость:', '')
        submitted = st.form_submit_button('Submit')
        if submitted:
            st.write("Парсинг ссылки...")
            answer = parse(link)
            st.success(f"Результат: {answer}")
