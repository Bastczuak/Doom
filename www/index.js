import * as doom from 'doom'

fetch('./Doom1.wad').then(async response => {
  const downloadedMap = await response.arrayBuffer()
  const map = doom.init(downloadedMap)
  console.log('###', map)
})
