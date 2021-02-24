import pandas as pd
from datetime import datetime

dateparse = lambda x: datetime.strptime(x, '%Y-%m-%dT%H:%M:%S')

df = pd.read_csv('dataset.csv', sep=";", parse_dates=['datetime'], date_parser=dateparse)
df.head()
dataplot = df.plot(x="datetime", y=["energy produced", "energy used"], color={"energy produced": 'green', "energy used": 'red'})
dataplot.set_xlabel("Time")
dataplot.set_ylabel("Energy (kWh)")
now = datetime.now() 
fig = dataplot.get_figure()
fig.savefig(now.strftime("%Y_%m_%d_%H_%M_%S.png"), dpi=200, bbox_inches = "tight")