async function main() {
  let kernel = await window.runCommand("kernel_name");
  let host = await window.runCommand("host_name");
  let user = await window.runCommand("user_name");
  let infoElement = document.getElementById("info");
  infoElement.innerHTML = `${user}<br/>${host}<br/>${kernel}`;
}

window.onload = main;
