use bevy::prelude::*;
use bevy_egui::{
    egui::{self, ScrollArea},
    EguiContext, EguiPlugin,
};
use bevy_mod_picking::prelude::*;
use egui_dock::{DockArea, NodeIndex, Style, Tree};

fn main() {
    let mut tree = Tree::new(vec!["tab1".to_owned(), "tab2".to_owned()]);

    // You can modify the tree before constructing the dock
    let [a, b] = tree.split_left(NodeIndex::root(), 0.3, vec!["tab3".to_owned()]);
    let [_, _] = tree.split_below(a, 0.7, vec!["tab4".to_owned()]);
    let [_, _] = tree.split_below(b, 0.5, vec!["tab5".to_owned()]);

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(bevy_framepace::FramepacePlugin) // significantly reduces input lag
        .add_plugin(EguiPlugin)
        .add_system(ui_example)
        .add_system(dock_example)
        .add_startup_system(setup)
        .insert_resource(MyTree(tree))
        .run();
}

fn ui_example(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
        ScrollArea::both().auto_shrink([false; 2]).show(ui, |ui| {
            ui.label("world");
        });
    });
}

struct TabViewer {}

impl egui_dock::TabViewer for TabViewer {
    type Tab = String;

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        ui.label(format!("Content of {tab}"));
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        (&*tab).into()
    }
}

#[derive(Resource)]
struct MyTree(Tree<String>);

fn dock_example(mut egui_context: ResMut<EguiContext>, mut tree: ResMut<MyTree>) {
    let ctx = egui_context.ctx_mut();
    DockArea::new(&mut tree.0)
        .style(Style::from_egui(ctx.style().as_ref()))
        .show(&ctx, &mut TabViewer {});
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: materials.add(Color::WHITE.into()),
            ..Default::default()
        },
        PickableBundle::default(),    // <- Makes the mesh pickable.
        PickRaycastTarget::default(), // <- Needed for the raycast backend.
    ));

    // cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..Default::default()
        },
        PickableBundle::default(),    // <- Makes the mesh pickable.
        PickRaycastTarget::default(), // <- Needed for the raycast backend.
    ));

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, -4.0),
        ..Default::default()
    });

    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(3.0, 3.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
            // Uncomment the following lines to try out orthographic projection:
            //
            // projection: bevy::render::camera::Projection::Orthographic(OrthographicProjection {
            //     scale: 0.01,
            //     ..Default::default()
            // }),
            ..Default::default()
        },
        PickRaycastSource::default(), // <- Enable picking for this camera
    ));
}
