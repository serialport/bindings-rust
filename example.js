const {
  Worker, isMainThread, parentPort, workerData
} = require('worker_threads');
const {list} = require('./')

if (isMainThread) {
  const worker = new Worker(__filename, { });
    worker.on('error', err => { throw err });
    worker.on('exit', (code) => {
      if (code !== 0)
        throw new Error(`Worker stopped with exit code ${code}`)
    });
} else {
  const example = async () => {
    console.log(JSON.stringify(await list(), null, 2))
  }
  example().catch(console.error)
}


