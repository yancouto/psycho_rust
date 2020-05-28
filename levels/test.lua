local F = Formations
local LM = LevelManager
local SB = Enemies.SimpleBall

for i = 1, 3 do
    LM.wait(1)
    F.single {
        enemy = SB,
        pos = vec2(-20, HEIGHT/2),
        speed = vec2(10, 0),
    }
end

LM.wait_until_no_enemies()
F.single {
    enemy = SB,
    pos = vec2(WIDTH / 2, -20),
    speed = vec2(0, 10),
}
