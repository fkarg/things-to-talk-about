>>> ids = encoder.encode("Not all heroes wear capes.")
>>> ids
[3673, 477, 10281, 5806, 1451, 274, 13]

>>> encoder.decode(ids)
"Not all heroes wear capes."

>>> [encoder.decode([i]) for i in ids]
['Not', ' all', ' heroes', ' wear', ' cap', 'es', '.']
