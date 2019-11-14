import * as doom from 'doom'
import * as THREE from 'three'

(async function run () {
  const response = await fetch('./Doom1.wad')
  const downloadedMap = await response.arrayBuffer()
  const map = doom.init(downloadedMap, 'E1M1')

  const xShift = -map.x_min - map.x_max / 2
  const yShift = -map.y_min + map.y_max / 2

  console.log('###', map, xShift, yShift)

  const scene = new THREE.Scene()
  map.line_defs.forEach(line => {
    const start = map.vertexes[line.start_vertex]
    const end = map.vertexes[line.end_vertex]
    const geometry = new THREE.Geometry()
    geometry.vertices.push(new THREE.Vector3(start.x + xShift, start.y + yShift, 0))
    geometry.vertices.push(new THREE.Vector3(start.x + xShift, start.y + yShift, 0))
    geometry.vertices.push(new THREE.Vector3(end.x + xShift, end.y + yShift, 0))
    geometry.vertices.push(new THREE.Vector3(end.x + xShift, end.y + yShift, 0))
    const material = new THREE.LineBasicMaterial({ color: 0xffffff })
    const l = new THREE.Line(geometry, material)
    scene.add(l)
  })

  const geometry = new THREE.CircleGeometry(25, 32)
  const material = new THREE.MeshBasicMaterial({ color: 0xff0000 })
  const player = new THREE.Mesh(geometry, material)
  player.position.set(map.player.x + xShift, map.player.y + yShift, 0)
  scene.add(player)

  const axesHelper = new THREE.AxesHelper(1000)
  scene.add(axesHelper)

  const scale = 0.025
  scene.scale.set(scale, scale, scale)

  const camera = new THREE.PerspectiveCamera(45, window.innerWidth / window.innerHeight, 0.1, 10000)
  camera.position.set(0, 0, 200)
  camera.lookAt(0, 0, 0)

  const renderer = new THREE.WebGLRenderer()
  renderer.setSize(window.innerWidth, window.innerHeight)
  document.body.appendChild(renderer.domElement)

  const animate = () => {
    requestAnimationFrame(animate)
    renderer.render(scene, camera)
  }
  animate()
})()
