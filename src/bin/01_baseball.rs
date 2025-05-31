use bevy::prelude::*; // Bevy의 필수 요소들을 가져옵니다.
use rand::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin { // 창 설정 예시 (선택 사항)
            primary_window: Some(Window {
                title: "Number Baseball Setup Test".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(SecretNumberSetupPlugin)
        .add_systems(Startup, print_secret_number_system) // 리소스 확인용 시스템 (선택 사항)
        .run();
}


// 3자리 숫자를 표현하는 컴포넌트 (정답, 사용자 추측 등)
#[derive(Component, Debug, Clone, PartialEq, Eq, Hash)] // Eq와 Hash도 필요할 수 있어 추가
pub struct ThreeDigitNumber([u8; 3]);

#[derive(Resource, Debug, Clone)]
pub struct SecretNumber {
    pub value: ThreeDigitNumber, // 실제 3자리 숫자를 저장하는 필드
}


pub struct SecretNumberSetupPlugin;

impl Plugin for SecretNumberSetupPlugin {
    fn build(&self, app: &mut App) {
        // 1. 랜덤 숫자 생성을 위한 준비
        let mut rng = rand::rng();
        let mut possible_digits: Vec<u8> = (0..=9).collect(); // 0부터 9까지의 숫자를 담은 벡터를 만듭니다.

        // 2. 숫자 목록을 랜덤하게 섞습니다.
        possible_digits.shuffle(&mut rng);

        // 3. 섞인 목록에서 앞에서부터 3개의 숫자를 선택하여 비밀번호로 사용합니다.
        // 이렇게 하면 항상 중복되지 않는 3개의 숫자를 얻을 수 있습니다.
        if possible_digits.len() < 3 {
            // 이 경우는 발생하지 않아야 하지만, 안전을 위해 처리 (0~9는 10개이므로 항상 3개 이상)
            error!("Failed to generate enough unique digits for the secret number.");
            // 기본값 또는 다른 오류 처리 로직을 여기에 추가할 수 있습니다.
            // 여기서는 간단히 기본값으로 [0,0,0]을 사용하고 오류 메시지를 출력합니다.
            app.insert_resource(SecretNumber {
                value: ThreeDigitNumber([0,0,0]),
            });
            return;
        }
        
        let secret_value = ThreeDigitNumber([
            possible_digits[0],
            possible_digits[1],
            possible_digits[2],
        ]);

        // 4. SecretNumber 리소스 인스턴스를 만듭니다.
        let secret_number_resource = SecretNumber { value: secret_value };
        
        // 5. 생성된 리소스를 앱에 추가합니다.
        app.insert_resource(secret_number_resource);
    }
}


// 이 예제 파일의 main 함수

// 리소스가 잘 들어갔는지 확인하는 간단한 시스템 (선택 사항)
fn print_secret_number_system(secret: Res<SecretNumber>) {
    info!("Current SecretNumber in resource: {:?} {:?} {:?}", secret.value.0[0], secret.value.0[1], secret.value.0[2]);
}