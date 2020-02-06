import { Doom, checkForSubSector, getSubSector } from 'doom'
import * as THREE from 'three'
import Stats from 'stats.js'

function addPlayer ({ x, y, angle, xShift, yShift }, scene) {
  const player = scene.getObjectByName('player')
  const positions = []
  const rad = angle * Math.PI / 180
  const halfFov = Math.PI / 4
  positions.push(
    x, y, 1,
    x + Math.cos(rad - halfFov) * 1000, y + Math.sin(rad - halfFov) * 1000, 1,
    x, y, 1,
    x + Math.cos(rad + halfFov) * 1000, y + Math.sin(rad + halfFov) * 1000, 1,
  )

  if (player) {
    player.position.set(x + xShift, y + yShift, 0)
    const fov = scene.getObjectByName('fov')
    if (fov) {
      fov.geometry.setAttribute('position', new THREE.Float32BufferAttribute(positions, 3))
    }
    return
  }

  let geometry = new THREE.CircleBufferGeometry(25, 32)
  let material = new THREE.MeshBasicMaterial({ color: 0xffffff })
  const mesh = new THREE.Mesh(geometry, material)
  mesh.position.set(x + xShift, y + yShift, 0)
  mesh.name = 'player'
  scene.add(mesh)

  geometry = new THREE.BufferGeometry()
  geometry.setAttribute('position', new THREE.Float32BufferAttribute(positions, 3))
  material = new THREE.LineBasicMaterial({ color: 0x00ff00 })
  const l = new THREE.LineSegments(geometry, material)
  l.position.set(xShift, yShift, 0)
  l.name = 'fov'
  scene.add(l)
}

function addMap ({ line_defs, vertexes, xShift, yShift }, scene) {
  const positions = []
  for (let line of line_defs) {
    const start = vertexes[line.start_vertex]
    const end = vertexes[line.end_vertex]
    positions.push(start.x, start.y, 0, end.x, end.y, 0)
  }

  const geometry = new THREE.BufferGeometry()
  geometry.setAttribute('position', new THREE.Float32BufferAttribute(positions, 3))
  const material = new THREE.LineBasicMaterial({ color: 0xffffff })
  const l = new THREE.LineSegments(geometry, material)
  l.position.set(xShift, yShift, 0)
  scene.add(l)
}

function renderVisibleVertexes ({ vertexes, xShift, yShift }, scene) {
  const visibleVertexes = scene.getObjectByName('visibleVertexes')
  const positions = []
  for (let v of vertexes) {
    const start = v['0']
    const end = v['1']
    positions.push(start.x, start.y, 1, end.x, end.y, 1)
  }

  if (visibleVertexes) {
    visibleVertexes.geometry.setAttribute('position', new THREE.Float32BufferAttribute(positions, 3))
    return
  }

  const geometry = new THREE.BufferGeometry()
  geometry.setAttribute('position', new THREE.Float32BufferAttribute(positions, 3))
  const material = new THREE.LineBasicMaterial({ color: 0xff0000 })
  const l = new THREE.LineSegments(geometry, material)
  l.position.set(xShift, yShift, 0)
  l.name = 'visibleVertexes'
  scene.add(l)
}

(async function run () {
  const scene = new THREE.Scene()
  const scale = 0.025
  scene.scale.set(scale, scale, scale)

  const camera = new THREE.PerspectiveCamera(45, window.innerWidth / window.innerHeight, 0.1, 10000)
  camera.position.set(0, 0, 150)
  camera.lookAt(scene.position)

  const renderer = new THREE.WebGLRenderer()
  renderer.setSize(window.innerWidth, window.innerHeight)
  document.body.appendChild(renderer.domElement)

  const response = await fetch('./doomu.wad')
  const downloadedMap = await response.arrayBuffer()

  const doom = Doom.new(downloadedMap)

  const map = doom.loadMap('E1M1')
  const xShift = -map.x_min - map.x_max / 2
  const yShift = -map.y_min + map.y_max / 2
  addMap({ ...map, xShift, yShift }, scene)
  doom.loadPlayer('E1M1', 1)

  const player = () => ({ ...doom.get_player()[0]['0'], ...doom.get_player()[0]['1'] })

  const stats = new Stats()
  document.body.appendChild(stats.dom)

  let pressedKey = ''

  document.addEventListener('keydown', e => {
    pressedKey = e.key
  })

  window.addEventListener('resize', () => {
    camera.aspect = window.innerWidth / window.innerHeight
    camera.updateProjectionMatrix()
    renderer.setSize(window.innerWidth, window.innerHeight)
  }, false)

  const animate = () => {
    stats.begin()
    doom.tick(pressedKey)
    addPlayer({ ...player(), xShift, yShift }, scene)
    renderVisibleVertexes({ ...doom.get_visible_vertexes(), xShift, yShift }, scene)
    renderer.render(scene, camera)
    pressedKey = ''
    stats.end()
    requestAnimationFrame(animate)
  }
  animate()
})()
