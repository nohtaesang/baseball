use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, Mesh},
        render_asset::RenderAssetUsages,
        render_resource::PrimitiveTopology,
    },
};

// HexMetrics는 사각형에 직접적으로 필요하지 않으므로, 이 함수에서는 사용하지 않습니다.
// 만약 다른 곳에서 HexMetrics를 계속 사용한다면, 해당 파일의 상단에는 use 구문이 남아있을 수 있습니다.

/// 주어진 한 변의 길이를 사용하여 XZ 평면에 놓인 사각형 메시를 생성합니다.
/// 사각형의 중심은 (0, 0, 0)입니다.
pub fn create_quad_mesh(side_length: f32) -> Mesh {
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList, // 여전히 삼각형 리스트로 메시를 구성합니다.
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );

    let half_size = side_length / 2.0;

    // 1. 정점 위치 (Vertex Positions) 정의
    // 사각형은 4개의 꼭짓점으로 구성됩니다. Y축(높이)은 0으로 설정하여 XZ 평면에 놓이도록 합니다.
    // 정점 순서: 아래왼쪽 -> 아래오른쪽 -> 위오른쪽 -> 위왼쪽 (반시계 방향)
    let positions = vec![
        [-half_size, 0.0, -half_size], // 0: 아래왼쪽 (Bottom-Left)
        [ half_size, 0.0, -half_size], // 1: 아래오른쪽 (Bottom-Right)
        [ half_size, 0.0,  half_size], // 2: 위오른쪽 (Top-Right)
        [-half_size, 0.0,  half_size], // 3: 위왼쪽 (Top-Left)
    ];
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions.clone());

    info!("Square positions: {:?}", positions);

    // 2. UV 좌표 (Texture Coordinates) 매핑
    // 텍스처의 (0,0)을 사각형의 아래왼쪽, (1,1)을 위오른쪽에 매핑합니다.
    let uvs = vec![
        [0.0, 0.0], // 0: 아래왼쪽
        [1.0, 0.0], // 1: 아래오른쪽
        [1.0, 1.0], // 2: 위오른쪽
        [0.0, 1.0], // 3: 위왼쪽
    ];
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    // 3. 법선 (Normals) 정의
    // 사각형은 평평하고 Y축 위를 향하므로, 모든 정점의 법선은 (0.0, 1.0, 0.0)입니다.
    let normals = vec![[0.0, 1.0, 0.0]; positions.len()];
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);

    // 4. 인덱스 (Indices) 정의
    // 4개의 정점을 사용하여 2개의 삼각형을 만듭니다.
    // 삼각형 1: 0-1-2 (아래왼쪽 - 아래오른쪽 - 위오른쪽)
    // 삼각형 2: 0-2-3 (아래왼쪽 - 위오른쪽 - 위왼쪽)
    // 이는 반시계 방향(Counter-Clockwise, CCW) 순서로, 기본적으로 앞면으로 간주됩니다.
    let indices = vec![
        0, 1, 2, // 첫 번째 삼각형
        0, 2, 3, // 두 번째 삼각형
    ];
    mesh.insert_indices(Indices::U32(indices));

    mesh
}