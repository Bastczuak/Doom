// import { doomInit, checkForSubSector, Doom } from 'doom'
//
// function addPlayer ({ player, xShift, yShift }, scene) {
//   const geometry = new THREE.CircleGeometry(25, 32)
//   const material = new THREE.MeshBasicMaterial({ color: 0xff0000 })
//   const mesh = new THREE.Mesh(geometry, material)
//   mesh.position.set(player.x + xShift, player.y + yShift, 0)
//   scene.add(mesh)
// }
//
function addMap ({ line_defs, vertexes, xShift, yShift }, scene) {
  line_defs.forEach(line => {
    const start = vertexes[line.start_vertex]
    const end = vertexes[line.end_vertex]
    const geometry = new THREE.Geometry()
    geometry.vertices.push(
      new THREE.Vector3(start.x, start.y, 0),
      new THREE.Vector3(end.x, end.y, 0),
    )
    const material = new THREE.LineBasicMaterial({ color: 0xffffff })
    const l = new THREE.Line(geometry, material)
    l.position.set(xShift, yShift, 0)
    scene.add(l)
  })
}

//
//
// function addNodes ({ nodes, xShift, yShift, isPointOnLeftSide }, scene) {
//   const recursive = (nodeIndex) => {
//     if (checkForSubSector(nodeIndex)) {
//       return
//     }
//
//     scene.remove(scene.getObjectByName('splitter'))
//     scene.remove(scene.getObjectByName('left'))
//     scene.remove(scene.getObjectByName('right'))
//
//     const {
//       x_partition,
//       y_partition,
//       change_x_partition,
//       change_y_partition,
//       right_box_top,
//       right_box_bottom,
//       right_box_right,
//       right_box_left,
//       left_box_top,
//       left_box_bottom,
//       left_box_right,
//       left_box_left,
//     } = nodes[nodeIndex]
//
//     let material = new THREE.LineBasicMaterial({ color: 0x00ff00 })
//     let geometry = new THREE.Geometry()
//     geometry.vertices.push(
//       new THREE.Vector3(right_box_left, right_box_top, 0),
//       new THREE.Vector3(right_box_right, right_box_top, 0),
//       new THREE.Vector3(right_box_right, right_box_bottom, 0),
//       new THREE.Vector3(right_box_left, right_box_bottom, 0),
//       new THREE.Vector3(right_box_left, right_box_top, 0),
//     )
//     let rect = new THREE.Line(geometry, material)
//     rect.position.set(xShift, yShift, 0)
//     rect.name = 'right'
//     scene.add(rect)
//
//     material = new THREE.LineBasicMaterial({ color: 0xff0000 })
//     geometry = new THREE.Geometry()
//     geometry.vertices.push(
//       new THREE.Vector3(left_box_left, left_box_top, 0),
//       new THREE.Vector3(left_box_right, left_box_top, 0),
//       new THREE.Vector3(left_box_right, left_box_bottom, 0),
//       new THREE.Vector3(left_box_left, left_box_bottom, 0),
//       new THREE.Vector3(left_box_left, left_box_top, 0),
//     )
//     rect = new THREE.Line(geometry, material)
//     rect.position.set(xShift, yShift, 0)
//     rect.name = 'left'
//     scene.add(rect)
//
//     geometry = new THREE.Geometry()
//     geometry.vertices.push(
//       new THREE.Vector3(x_partition, y_partition, 0),
//       new THREE.Vector3(x_partition + change_x_partition, y_partition + change_y_partition, 0),
//     )
//     material = new THREE.LineBasicMaterial({ color: 0x77c9f9 })
//     const line = new THREE.Line(geometry, material)
//     line.position.set(xShift, yShift, 0)
//     line.name = 'splitter'
//     scene.add(line)
//
//     setTimeout(() => {
//       if (isPointOnLeftSide(nodeIndex)) {
//         recursive(nodes[nodeIndex].left_child)
//       } else {
//         recursive(nodes[nodeIndex].right_child)
//       }
//     }, 1000)
//   }
//   return recursive
// }
//
// function traverseBspTree ({ nodes, player, xShift, yShift }, scene) {
//   const isPointOnLeftSide = (nodeIndex) => {
//     const dx = player.x - nodes[nodeIndex].x_partition
//     const dy = player.y - nodes[nodeIndex].y_partition
//     return (((dx * nodes[nodeIndex].change_y_partition) - (dy * nodes[nodeIndex].change_x_partition)) <= 0)
//   }
//   const recursive = addNodes({ nodes, xShift, yShift, isPointOnLeftSide }, scene)
//   const startIndex = nodes.length - 1
//   recursive(startIndex)
// }
//
// (async function run () {
//   const doom = Doom.new(x => {
//     console.log('@@@', x)
//   })
//   doom.tick()
//   const response = await fetch('./doom1.wad')
//   const downloadedMap = await response.arrayBuffer()
//   const map = doomInit(downloadedMap, 'E1M1')
//
//   const xShift = -map.x_min - map.x_max / 2
//   const yShift = -map.y_min + map.y_max / 2
//
//   console.log('###', map, xShift, yShift)
//
//   const scene = new THREE.Scene()
//
//   const scale = 0.025
//   scene.scale.set(scale, scale, scale)
//
//   const camera = new THREE.PerspectiveCamera(45, window.innerWidth / window.innerHeight, 0.1, 10000)
//   camera.position.set(0, 0, 150)
//   camera.lookAt(0, 0, 0)
//
//   const renderer = new THREE.WebGLRenderer()
//   renderer.setSize(window.innerWidth, window.innerHeight)
//   document.body.appendChild(renderer.domElement)
//
//   window.addEventListener('resize', () => {
//     camera.aspect = window.innerWidth / window.innerHeight
//     camera.updateProjectionMatrix()
//     renderer.setSize(window.innerWidth, window.innerHeight)
//   }, false)
//
//   const animate = () => {
//     requestAnimationFrame(animate)
//     renderer.render(scene, camera)
//   }
//
//   addMap({ ...map, xShift, yShift }, scene)
//   addPlayer({ ...map, xShift, yShift }, scene)
//   traverseBspTree({ ...map, xShift, yShift }, scene)
//
//   animate()
// })()

import { Doom } from 'doom'
import * as THREE from 'three'

(async function run () {
  const scene = new THREE.Scene()
  const scale = 0.025
  scene.scale.set(scale, scale, scale)

  const camera = new THREE.PerspectiveCamera(45, window.innerWidth / window.innerHeight, 0.1, 10000)
  camera.position.set(0, 0, 150)
  camera.lookAt(0, 0, 0)

  const renderer = new THREE.WebGLRenderer()
  renderer.setSize(window.innerWidth, window.innerHeight)
  document.body.appendChild(renderer.domElement)

  window.addEventListener('resize', () => {
    camera.aspect = window.innerWidth / window.innerHeight
    camera.updateProjectionMatrix()
    renderer.setSize(window.innerWidth, window.innerHeight)
  }, false)

  const animate = () => {
    requestAnimationFrame(animate)
    renderer.render(scene, camera)
  }

  animate()

  const response = await fetch('./doom1.wad')
  const downloadedMap = await response.arrayBuffer()
  const doom = Doom.new(downloadedMap)
  doom.load('E1M1', map => {
    const xShift = -map.x_min - map.x_max / 2
    const yShift = -map.y_min + map.y_max / 2

    console.log('###', map, xShift, yShift)

    addMap({ ...map, xShift, yShift }, scene)
  })
})()
