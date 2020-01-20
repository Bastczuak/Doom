import { Doom, checkForSubSector, getSubSector } from 'doom'
import * as THREE from 'three'

function addPlayer ({ x, y, angle, xShift, yShift }, scene) {
  const player = scene.getObjectByName('player')
  if (player) {
    player.position.set(x + xShift, y + yShift, 0)
    player.rotation.z = angle * Math.PI / 180
    return
  }

  const geometry = new THREE.Geometry()
  const v1 = new THREE.Vector3(-30, 0, 0)
  const v2 = new THREE.Vector3(30, 0, 0)
  const v3 = new THREE.Vector3(0, 60, 0)
  geometry.vertices.push(v1)
  geometry.vertices.push(v2)
  geometry.vertices.push(v3)
  geometry.faces.push(new THREE.Face3(0, 1, 2))
  geometry.computeFaceNormals()
  const mesh = new THREE.Mesh(geometry, new THREE.MeshNormalMaterial())
  mesh.position.set(x + xShift, y + yShift, 0)
  mesh.name = 'player'
  scene.add(mesh)
}

let pressedKey = ''

document.addEventListener('keydown', e => {
  pressedKey = e.key
})

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

function sleep (ms) {
  return new Promise(resolve => setTimeout(resolve, ms))
}

function addNodes ({ nodes, segs, ssectors, vertexes, xShift, yShift, isPointOnLeftSide }, scene) {
  const recursive = async (nodeIndex) => {
    if (checkForSubSector(nodeIndex)) {
      const ssector = ssectors[getSubSector(nodeIndex)]
      traverseSectors({ segs, ssector, vertexes, yShift, xShift }, scene)
      return
    }

    scene.remove(scene.getObjectByName('splitter'))
    scene.remove(scene.getObjectByName('left'))
    scene.remove(scene.getObjectByName('right'))

    const {
      x_partition,
      y_partition,
      change_x_partition,
      change_y_partition,
      right_box_top,
      right_box_bottom,
      right_box_right,
      right_box_left,
      left_box_top,
      left_box_bottom,
      left_box_right,
      left_box_left,
    } = nodes[nodeIndex]

    let material = new THREE.LineBasicMaterial({ color: 0x00ff00 })
    let geometry = new THREE.Geometry()
    geometry.vertices.push(
      new THREE.Vector3(right_box_left, right_box_top, 0),
      new THREE.Vector3(right_box_right, right_box_top, 0),
      new THREE.Vector3(right_box_right, right_box_bottom, 0),
      new THREE.Vector3(right_box_left, right_box_bottom, 0),
      new THREE.Vector3(right_box_left, right_box_top, 0),
    )
    let rect = new THREE.Line(geometry, material)
    rect.position.set(xShift, yShift, 0)
    rect.name = 'right'
    scene.add(rect)

    material = new THREE.LineBasicMaterial({ color: 0xff0000 })
    geometry = new THREE.Geometry()
    geometry.vertices.push(
      new THREE.Vector3(left_box_left, left_box_top, 0),
      new THREE.Vector3(left_box_right, left_box_top, 0),
      new THREE.Vector3(left_box_right, left_box_bottom, 0),
      new THREE.Vector3(left_box_left, left_box_bottom, 0),
      new THREE.Vector3(left_box_left, left_box_top, 0),
    )
    rect = new THREE.Line(geometry, material)
    rect.position.set(xShift, yShift, 0)
    rect.name = 'left'
    scene.add(rect)

    geometry = new THREE.Geometry()
    geometry.vertices.push(
      new THREE.Vector3(x_partition, y_partition, 0),
      new THREE.Vector3(x_partition + change_x_partition, y_partition + change_y_partition, 0),
    )
    material = new THREE.LineBasicMaterial({ color: 0x77c9f9 })
    const line = new THREE.Line(geometry, material)
    line.position.set(xShift, yShift, 0)
    line.name = 'splitter'
    scene.add(line)

    await sleep(100)

    if (isPointOnLeftSide(nodeIndex)) {
      await recursive(nodes[nodeIndex].left_child)
      await recursive(nodes[nodeIndex].right_child)
    } else {
      await recursive(nodes[nodeIndex].right_child)
      await recursive(nodes[nodeIndex].left_child)
    }
  }
  return recursive
}

async function traverseBspTree ({ nodes, player, xShift, yShift, segs, ssectors, vertexes }, scene) {
  const isPointOnLeftSide = (nodeIndex) => {
    const dx = player.x - nodes[nodeIndex].x_partition
    const dy = player.y - nodes[nodeIndex].y_partition
    return (((dx * nodes[nodeIndex].change_y_partition) - (dy * nodes[nodeIndex].change_x_partition)) <= 0)
  }
  const recursive = addNodes({ nodes, xShift, yShift, isPointOnLeftSide, segs, ssectors, vertexes }, scene)
  const startIndex = nodes.length - 1
  await recursive(startIndex)
}

function traverseSectors ({ segs, ssector, vertexes, xShift, yShift }, scene) {
  for (let i = 0; i < ssector.seg_count; i++) {
    const seg = segs[ssector.first_seg + i]
    const start = vertexes[seg.start_vertex]
    const end = vertexes[seg.end_vertex]
    const geometry = new THREE.Geometry()
    geometry.vertices.push(
      new THREE.Vector3(start.x, start.y, 0),
      new THREE.Vector3(end.x, end.y, 0),
    )
    const material = new THREE.LineBasicMaterial({ color: 0xffc0cb })
    const l = new THREE.Line(geometry, material)
    l.position.set(xShift, yShift, 0)
    scene.add(l)
  }
}

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

  const response = await fetch('./doomu.wad')
  const downloadedMap = await response.arrayBuffer()

  const doom = Doom.new(downloadedMap)

  const map = doom.loadMap('E1M1')
  const xShift = -map.x_min - map.x_max / 2
  const yShift = -map.y_min + map.y_max / 2
  addMap({ ...map, xShift, yShift }, scene)
  doom.loadPlayer('E1M1', 1)

  const animate = () => {
    doom.tick(pressedKey)
    const player = doom.get_player()[0]
    addPlayer({ ...player['0'], ...player['1'], xShift, yShift }, scene)
    renderer.render(scene, camera)
    pressedKey = ''
    requestAnimationFrame(animate)
  }
  animate()
})()
