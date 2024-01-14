import sympy as sp

"""
If we take 3 hailstones then we have 9 eqs and 9 unknowns:

Vxs*t0 + Pxs = Vx0*t0 + Px0 # Hailstone 0 eqs
Vys*t0 + Pys = Vy0*t0 + Py0
Vzs*t0 + Pzs = Vz0*t0 + Pz0
Vxs*t1 + Pxs = Vx1*t1 + Px1 # Hailstone 1 eqs
Vys*t1 + Pys = Vy1*t1 + Py1
Vzs*t1 + Pzs = Vz1*t1 + Pz1
Vxs*t2 + Pxs = Vx2*t2 + Px2 # Hailstone 2 eqs
Vys*t2 + Pys = Vy2*t2 + Py2
Vzs*t2 + Pzs = Vz2*t2 + Pz2

Unknowns: {Pxs, Pys, Pzs, Vxs, Vys, Vzs, t0, t1, t2}

A bit of algebra (solving for t1, t2, t3) can reduce this problem to the
following 6 eqs and 6 unknowns:

(Px0 - Pxs) / (Vxs - Vx0) = (Py0 - Pys) / (Vys - Vy0) = (Pz0 - Pzs) / (Vzs - Vz0)
(Px1 - Pxs) / (Vxs - Vx1) = (Py1 - Pys) / (Vys - Vy1) = (Pz1 - Pzs) / (Vzs - Vz1)
(Px2 - Pxs) / (Vxs - Vx2) = (Py2 - Pys) / (Vys - Vy2) = (Pz2 - Pzs) / (Vzs - Vz2)

I thought about doing some kind of clever filtering via the constraint that all
the t values must be > 0, but gave up on that and just let sympy do the work.
"""

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
