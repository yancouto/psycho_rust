local F = Formations
local LM = LevelManager
local SB = Enemies.SimpleBall

for i = 1, 3 do
    LM.wait(1)
    F.single {
        enemy = SB,
        pos = vec2(20, 100),
        speed = vec2(10, 0),
    }
end
