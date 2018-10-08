window.onload = function () {
  initGrapesJs();
};

// window.onbeforeunload = function () {
//   storeGrapesJsData();
// };

function initGrapesJs() {
  var thisuri = window.location.href;
  thisuri = thisuri.replace("/build/", "/store/");

  const editor = grapesjs.init({
    container: '#gjs',
    height: '800px',
    noticeOnUnload: 0,
    // fromElement: true,
    //    storageManager: { type: null },
    plugins: ['gjs-preset-newsletter'],
    pluginsOpts: {
      'gjs-preset-newsletter': {
        modalTitleImport: 'Import template',
        // ... other options
      }
    },
    storageManager: {
      type: 'ws-storage',
      stepsBeforeSave: 3,
      autoload: true,
    },
    // storageManager: {
    //   id: 'gjs-nl-',
    //   type: 'remote',
    //   // stepsBeforeSave: 1,
    //   autosave: false,
    //   urlStore: thisuri,
    //   urlLoad: thisuri,
    //   // storeComponents: true,
    //   // storeCss: true,
    //   // storeHtml: true,
    //   // storeStyles: true,
    //   contentTypeJson: true,
    //   // For custom parameters/headers on requests
    //   // params: { _some_token: '....' },
    //   // headers: { Authorization: 'Basic ...' },
    // },
    assetManager: {
      upload: 0,
      uploadText: 'Put url to your image to the textbox on the right, then press Add image.',
    },
  });
  var panelManager = editor.Panels;
  var save_button = panelManager.addButton('options', {
    id: 'save-now',
    className: 'fa fa-floppy-o icon-blank',
    command: 'save-now',
    attributes: {
      'title': 'Save changes',
      'data-tooltip-pos': 'bottom',
    },
  });
  var commands = editor.Commands;
  commands.add('save-now',
    function(editor, sender) {
      // store json
      // if(editor.getDirtyCount() != 0) {
      editor.store(res => {
        console.log('Store callback');
        if(sender) sender.set('active',false);
      });
      // }
    }
  );

  // Here our `ws-storage` implementation
  var load_clb;
  var ws_endpoint = window.location.pathname.replace("/build/", "/store/");
  var wsUri = (window.location.protocol=='https:'&&'wss://'||'ws://')+window.location.host + ws_endpoint;
  // wsUri = "wss://echo.websocket.org";
  var conn = new WebSocket(wsUri);
  conn.onopen = function() {
    console.log('Grape-Ws connected: ' + conn.protocol);
  };
  conn.onmessage = function(e) {
    console.log('Grape-Ws received: ' + e.data);
    var isString = e.data && typeof e.data === 'string';
    if (isString) {
      var result = JSON.parse(e.data);
    }
    load_clb(result);
  };
  conn.onclose = function() {
    console.log('Grape-Ws session disconnected.');
    conn = null;
    const modal = editor.Modal;
    modal.open({ title: "Warning", content: "Disconnected from email editor service."});
  };
  conn.onerror = function(evt) { 
    console.log('Grape-Ws error: ' + evt);
  };

  editor.StorageManager.add('ws-storage', {
    /**
     * Load the data
     * @param  {Array} keys Array containing values to load, eg, ['gjs-components', 'gjs-style', ...]
     * @param  {Function} clb Callback function to call when the load is ended
     * @param  {Function} clbErr Callback function to call in case of errors
     */
    load(keys, clb, clbErr) {
      conn.send("load");
      console.log('Grape-Ws load...');
      load_clb = clb;
    },

    /**
     * Store the data
     * @param  {Object} data Data object to store
     * @param  {Function} clb Callback function to call when the load is ended
     * @param  {Function} clbErr Callback function to call in case of errors
     */
    store(data, clb, clbErr) {
      var toStore = JSON.stringify(data);
      if(conn == null) conn = new WebSocket(wsUri);
      conn.send(toStore);
      // Might be called inside some async method
      clb();
    }
  });
  setTimeout(editor.load,1000)
}
