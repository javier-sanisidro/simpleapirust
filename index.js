// Add this to the VERY top of the first file loaded in your app
var apm = require('elastic-apm-node').start({
  // Override service name from package.json
  // Allowed characters: a-z, A-Z, 0-9, -, _, and space
  serviceName: 'detectado',
  serverUrl: 'http://localhost:8200'
});

  const app = require('express')()
  app.get('/', function (req, res) {
    res.send('Hello World!')

  })

  
  app.listen(3000)