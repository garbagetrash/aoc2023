import sympy as sp

pxs = sp.symbols('pxs')
vxs = sp.symbols('vxs')
pys = sp.symbols('pys')
vys = sp.symbols('vys')
pzs = sp.symbols('pzs')
vzs = sp.symbols('vzs')

px0 = 181274863478376
py0 = 423998359962919
pz0 = 286432452709141
vx0 = -104
vy0 = -373
vz0 = -52

px1 = 226461907371205
py1 = 306634733438686
pz1 = 305056780555025
vx1 = 54
vy1 = 35
vz1 = -49

px2 = 347320263466693
py2 = 360139618479358
pz2 = 271232232403985
vx2 = -63
vy2 = -122
vz2 = 26

eq1 = (px0 - pxs) / (vxs - vx0) - (py0 - pys) / (vys - vy0)
eq2 = (px0 - pxs) / (vxs - vx0) - (pz0 - pzs) / (vzs - vz0)
eq3 = (px1 - pxs) / (vxs - vx1) - (py1 - pys) / (vys - vy1)
eq4 = (px1 - pxs) / (vxs - vx1) - (pz1 - pzs) / (vzs - vz1)
eq5 = (px2 - pxs) / (vxs - vx2) - (py2 - pys) / (vys - vy2)
eq6 = (px2 - pxs) / (vxs - vx2) - (pz2 - pzs) / (vzs - vz2)

ans = sp.solve([eq1, eq2, eq3, eq4, eq5, eq6], dict=True)[0]

answer = ans[pxs] + ans[pys] + ans[pzs]
print(f"Solution: {answer}")
