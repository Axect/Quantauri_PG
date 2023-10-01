import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
import scienceplots

# Import parquet file
df = pd.read_parquet('data/bollinger.parquet')
df = df[(df.index >= 5000)]

# Prepare Data to Plot
tp = df['tp']
ubb = df['ubb']
mbb = df['mbb']
lbb = df['lbb']
perb = df['perb']
bw = df['bw']
date = np.arange(len(tp))

# Plot params
pparam = dict(
    xlabel = r'Index',
    ylabel = r'Value',
    xscale = 'linear',
    yscale = 'linear',
)

# Plot
with plt.style.context(["science", "nature"]):
    fig, axs = plt.subplots(3, 1, figsize=(6, 9), sharex=True)
    axs[0].autoscale(tight=True)
    axs[0].plot(date, tp, 'k-', label='Typical Price')
    axs[0].plot(date, ubb, 'r--', label='Upper Bollinger Band')
    axs[0].plot(date, mbb, 'g--', label='Middle Bollinger Band')
    axs[0].plot(date, lbb, 'b--', label='Lower Bollinger Band')
    axs[0].set_xlabel('Date')
    axs[0].set_ylabel('Price')

    axs[1].autoscale(tight=True)
    axs[1].plot(date, perb, 'k-', label='Percent B')
    axs[1].axhline(y=1, color='r', linestyle='--', label='Overbought')
    axs[1].axhline(y=0.5, color='g', linestyle='--', label='Middle')
    axs[1].axhline(y=0, color='b', linestyle='--', label='Oversold')
    axs[1].set_xlabel('Date')
    axs[1].set_ylabel('Percent B')

    axs[2].autoscale(tight=True)
    axs[2].plot(date, bw, 'k-', label='Bandwidth')
    axs[2].set_xlabel('Date')
    axs[2].set_ylabel('Bandwidth')

    for ax in axs:
        ax.legend()
        ax.grid(True)

    fig.savefig('figs/bollinger.png', dpi=300, bbox_inches='tight')
