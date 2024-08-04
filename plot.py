from matplotlib import pylab as plt
import sqlite3
import numpy as np

con = sqlite3.connect("db.sqlite3")
cur = con.cursor()
query = "SELECT value, read_at FROM readings "
query += "WHERE session_id = (SELECT max(id) FROM sessions) "
query += "ORDER BY id ASC"
res = cur.execute(query)
res = res.fetchall()

values, timestamps = zip(*res)
values = np.array(values, dtype=float)
timestamps = np.array(timestamps, dtype=float) / 1000

# reduce resolution
values = values[::20]
timestamps = timestamps[::20]

# calculate velocity
x = (timestamps[:-1] + timestamps[1:]) / 2
y = np.diff(values) / np.diff(timestamps)

plt.plot(x, y)
plt.savefig("plot.pdf")
