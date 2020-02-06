use crate::angle::Angle;
use crate::component::*;
use crate::wad::node::Node;
use crate::wad::seg::Seg;
use crate::wad::ssector::SSector;
use crate::wad::vertex::{Vertex, VisibleVertexes};
use specs::prelude::*;
use std::f32::consts::PI;

struct ViewFinder<'a> {
  nodes: &'a Vec<Node>,
  segs: &'a Vec<Seg>,
  ssectros: &'a Vec<SSector>,
  vertexes: &'a Vec<Vertex>,
  player: (&'a Rotation, &'a Position, &'a KeyboardControlled),
  result: VisibleVertexes,
}

impl<'a> ViewFinder<'a> {
  fn check_for_sub_sector(node_id: usize) -> bool {
    const SUB_SECTOR_IDENTIFIER: usize = 0x8000;
    node_id & SUB_SECTOR_IDENTIFIER > 0
  }

  fn get_sub_sector(node_id: usize) -> usize {
    const SUB_SECTOR_IDENTIFIER: usize = 0x8000;
    node_id & !SUB_SECTOR_IDENTIFIER
  }


  fn is_point_on_left_side(&self, node_id: usize) -> bool {
    let node = &self.nodes[node_id];
    let pos = self.player.1;
    let dx = pos.x - node.x_partition;
    let dy = pos.y - node.y_partition;
    (((dx * node.change_y_partition) - (dy * node.change_x_partition)) <= 0)
  }

  fn vertex_to_angle(&self, vertex: &Vertex) -> Angle {
    let position = self.player.1;
    let dx = f32::from(vertex.x - position.x);
    let dy = f32::from(vertex.y - position.y);
    Angle::new(dy.atan2(dx) * 180.0 / PI)
  }

  fn clip_vertexes_in_fov(&self, v1: &Vertex, v2: &Vertex) -> bool {
    let rot = self.player.0;
    let fov = f32::from(rot.fov);
    let mut v1_angle = self.vertex_to_angle(v1);
    let mut v2_angle = self.vertex_to_angle(v2);
    v2_angle = v2_angle - rot.angle;
    let half_fov = fov / 2.0;
    let mut v1_moved = v1_angle + half_fov;

    if v1_moved > fov {
      v1_moved = v1_moved - fov;

      if v1_moved >= delta {
        return false;
      }
    }
    true
  }

  fn find_vertexes_in_sub_sector(&mut self, ssector_id: usize) {
    let ssector = &self.ssectros[ssector_id];
    for i in 0..ssector.seg_count {
      let seg = &self.segs[(ssector.first_seg + i) as usize];
      let v1 = &self.vertexes[seg.start_vertex as usize];
      let v2 = &self.vertexes[seg.end_vertex as usize];

      if self.clip_vertexes_in_fov(v1, v2) {
        self.result.vertexes.push((*v1, *v2));
      }
    }
  }

  fn bsp(&mut self, node_id: usize) {
    if ViewFinder::check_for_sub_sector(node_id) {
      self.find_vertexes_in_sub_sector(ViewFinder::get_sub_sector(node_id));
      return;
    }

    if self.is_point_on_left_side(node_id) {
      self.bsp(self.nodes[node_id].left_child as usize);
      self.bsp(self.nodes[node_id].right_child as usize);
    } else {
      self.bsp(self.nodes[node_id].right_child as usize);
      self.bsp(self.nodes[node_id].left_child as usize);
    }
  }
}


pub struct View;

impl<'a> System<'a> for View {
  type SystemData = (
    ReadStorage<'a, Rotation>,
    ReadStorage<'a, Position>,
    ReadStorage<'a, KeyboardControlled>,
    ReadExpect<'a, Vec<Node>>,
    ReadExpect<'a, Vec<Seg>>,
    ReadExpect<'a, Vec<SSector>>,
    ReadExpect<'a, Vec<Vertex>>,
    WriteExpect<'a, VisibleVertexes>,
  );

  fn run(&mut self, mut data: Self::SystemData) {
    let nodes = &*data.3;
    let segs = &*data.4;
    let ssectros = &*data.5;
    let vertexes = &*data.6;
    let player = (&data.0, &data.1, &data.2).join().collect::<Vec<_>>()[0];
    let mut view_finder = ViewFinder {
      nodes,
      segs,
      ssectros,
      vertexes,
      player,
      result: Default::default(),
    };
    view_finder.bsp(nodes.len() - 1);
    *data.7 = view_finder.result;
  }
}
