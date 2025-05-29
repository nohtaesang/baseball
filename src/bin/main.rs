use bevy::prelude::*; // Bevy의 필수 요소들을 가져옵니다.

#[derive(Component)] // 이 구조체가 컴포넌트임을 나타냅니다.
struct Person;

#[derive(Component)]
struct Name(String); // 이름을 저장하는 Name 컴포넌트. 튜플 구조체(tuple struct) 형태입니다.

// 이전에 만들었던 시스템 함수
fn hello_world() {
    println!("hello world!");
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // Bevy의 기본 플러그인 그룹을 추가합니다.
        .add_plugins(HelloPlugin) // 새로 만든 HelloPlugin을 추가합니다.
       
        .run();
}

fn add_people(mut commands: Commands) {
    // commands.spawn()를 호출하여 새 엔티티를 생성하고,
    // 튜플 형태로 컴포넌트들을 전달하여 해당 엔티티에 추가합니다.
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

// fn greet_people(query: Query<&Name, With<Person>>) {
//     // 쿼리 결과를 순회하며 각 Name 컴포넌트의 값을 사용합니다.
//     for name in query.iter() { // query.iter() 또는 &query로 반복 가능
//         // Name 컴포넌트는 Name(String) 형태의 튜플 구조체이므로,
//         // 내부 String 값에 접근하기 위해 name.0을 사용합니다.
//         println!("hello {}!", name.0);
//     }
// }

// greet_people 시스템이 Time 리소스에 접근하도록 수정합니다.
// fn greet_people(time: Res<Time>, /* mut timer, */ query: Query<&Name, With<Person>>) {
//     // time.delta() 등을 사용하여 시간 정보에 접근할 수 있습니다.

//     println!("time: {:?}", time.delta());

//     for name in query.iter() {
//         println!("hello {}! (Time: {}s)", name.0, time.elapsed_secs()); // 예시로 현재 시간 출력
//     }
// }


fn update_people(mut query: Query<&mut Name, With<Person>>) {
    // 변경 가능한 쿼리 결과를 순회합니다.
    // name도 mut로 받아야 내부 값을 변경할 수 있습니다.
    for mut name in query.iter_mut() { // 또는 &mut query 로 반복 가능
        if name.0 == "Elaina Proctor" {
            name.0 = "Elaina Hume 태상".to_string(); // 이름을 변경합니다.
            break; // 다른 이름은 변경할 필요가 없으므로 루프를 종료합니다.
        }
    }
}


pub struct HelloPlugin; // 우리 플러그인을 위한 빈 구조체

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
         // GreetTimer 리소스를 앱에 추가(삽입)합니다.
        // 2.0초 간격으로 반복되는 타이머로 설정합니다.
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));

        app.add_systems(Startup, add_people);
        // hello_world 시스템은 제거했으므로, update_people과 greet_people만 남습니다.
        app.add_systems(Update, (update_people, greet_people).chain());
    }
}

#[derive(Resource)] // 이 구조체가 리소스임을 나타냅니다.
struct GreetTimer(Timer); // 내부에 Bevy의 Timer 타입을 가집니다.

// greet_people 시스템을 Time 리소스와 새로 만든 GreetTimer 리소스를 사용하도록 수정합니다.
fn greet_people(
    time: Res<Time>,                // 현재 시간 정보에 접근
    mut timer: ResMut<GreetTimer>,  // GreetTimer 리소스에 변경 가능한 접근
    query: Query<&Name, With<Person>> // 기존 쿼리
) {
    // timer 내부의 Timer를 업데이트하고, 타이머가 방금 완료되었는지 확인합니다.
    // timer.0은 GreetTimer 튜플 구조체의 첫 번째 필드인 Timer에 접근합니다.
    if timer.0.tick(time.delta()).just_finished() {
        // 타이머가 완료되었다면 모든 사람에게 인사합니다.
        for name in query.iter() {
            println!("hello {}!", name.0);
        }
    }
}