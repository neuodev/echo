module.exports = {
  apps: [
    {
      name: "pokemon",
      script: "./target/release/ehco",
      exec_interpreter: "none",
      exec_mode: "fork_mode",
    },
  ],
};
