
window.$koda = (() => {
  const params = new URLSearchParams(window.location.search);
  const debug = params.has('debug');
  let hash = params.get('hash');
  function randomhash() {
    let str = "0x";
    for (let i = 0; i < 64; i++) {
      str += Math.floor(Math.random() * 16).toString(16);
    }
    return str;
  }
  if (debug) {
    if (!hash) {
      hash = randomhash();
    }
    document.onclick = () => {
      window.location.search = "?debug=1&hash=" + randomhash();
    };
  }
  if (!hash) {
    console.error("hash must be provided in query string OR set debug=1");
  }

  function saveImagePNG(imageDataURL) {
    console.log('Talking');
    let message = {
      id: Date.now(),
      type: 'kodahash/render/completed',
      payload: {
        hash: hash,
        type: 'image/png',
        image: imageDataURL,
        search: location.search,
        attributes: []
      }
    };
    console.log('Sending', message);
    if (window.parent) window.parent.postMessage(message, "*");
  }

  return {
    debug,
    hash,
    saveImagePNG,
    features(_props) {
      // not currently supported by the platform?
    },
  };
})();