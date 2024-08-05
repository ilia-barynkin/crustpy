import pyvisa
import numpy as np
import matplotlib.pyplot as plt
from matplotlib.animation import FuncAnimation

# Создаем ресурс-менеджер
rm = pyvisa.ResourceManager()

# Подключаемся к осциллографу через TCP/IP
# Замените '192.168.1.100' на IP-адрес вашего осциллографа
instrument = rm.open_resource('TCPIP0::192.168.29.3::INSTR')

# Устанавливаем необходимые параметры осциллографа
instrument.write(":WAV:SOUR CHAN1")
instrument.write(":WAV:MODE NORM")
instrument.write(":WAV:FORM ASC")

# Инициализация графика
fig, ax = plt.subplots()
line, = ax.plot([], [], lw=2)

# Устанавливаем пределы осей (при необходимости их можно будет менять динамически)
ax.set_xlim(0, 1200)  # Пределы оси X
ax.set_ylim(0, 255)   # Пределы оси Y для 8-битных данных

def init():
    """Инициализация графика."""
    line.set_data([], [])
    return line,

def update(frame):
    """Функция для обновления графика."""
    instrument.write(":WAV:DATA?")
    data = instrument.read_raw()

    # Обработка данных
    header_len = 2 + int(data[1])
    header = data[:header_len]
    waveform_data = data[header_len:-1]
    waveform = np.frombuffer(waveform_data, dtype='B')

    # Обновление данных графика
    xdata = np.linspace(0, len(waveform) - 1, len(waveform))
    ydata = waveform
    line.set_data(xdata, ydata)
    return line,

def on_close(event):
    """Функция для закрытия соединения при закрытии окна."""
    instrument.close()
    print("Connection closed.")

# Создание анимации
ani = FuncAnimation(fig, update, init_func=init, blit=True, interval=10)

# Добавляем обработчик закрытия окна
fig.canvas.mpl_connect('close_event', on_close)

# Отображение графика
plt.show()