import pandas as pd
import matplotlib.pyplot as plt

df = pd.read_parquet('test.parquet')
df = df[df.index > 5000]

plt.figure(figsize=(10, 6), dpi=150)

df.data.plot(style='k--')
df.sma.plot(style='g')
df.ema.plot(style='orange')
df.bb_up.plot(style='r-.')
df.bb_down.plot(style='b-.')

plt.savefig('bollinger.png')

plt.figure(figsize=(10, 6), dpi=150)

df.macd.plot(style='r')
df.signal.plot(style='b--')

plt.savefig('macd.png')
