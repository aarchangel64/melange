console.log("AHHHHH");
window.functions = {};

window.runCommand = (command) => {
  let promise = new Promise((resolve, reject) => {
    window.functions[command] = resolve;
  });

  window.ipc.postMessage(command);
  return promise;
};
