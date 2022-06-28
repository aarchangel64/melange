window.functions = {};

window.runCommand = (command) => {
  let promise = new Promise((resolve, reject) => {
    window.functions[command] = resolve;
  });

  window.ipc.postMessage(command);
  return promise;
};

async function main() {
  let output = await window.runCommand("echo");
  console.log(output);
}

window.onload = main;
