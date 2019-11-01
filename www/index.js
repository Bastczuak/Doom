import * as doom from 'doom'

fetch('./Doom1.wad').then(async response => {
  let downloadedMap = await response.arrayBuffer()
  let map = doom.init(downloadedMap)
  console.log('###', map)
})
