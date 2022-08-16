module.exports = {
  apps: [
    {
      name: "pokemon",
      script: "./target/release/echo",
      exec_interpreter: "none",
      exec_mode: "fork_mode",
    },
  ],
};
