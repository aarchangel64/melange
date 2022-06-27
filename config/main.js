window.response = (stdout) => {
  console.log(stdout);
};

window.ipc.postMessage("echo");
