<script>
  import { Link } from "@inertiajs/inertia-svelte";
  export let user;
</script>

<div class="sm:px-8 mx-auto text-center">
  <h1 class="text-4xl mx-auto p-8">Welcome to Automata Games</h1>
  <p>
    You are logged in as <code class="code">{user}</code>.
  </p>

  <h1 class="text-xl mt-4 p-2">Mini Apps</h1>
  <div class="flex flex-wrap gap-4">
    <div class="w-25 btn btn-primary">
      <Link href="/chat">Chat</Link>
    </div>
    <div class="w-25 btn btn-primary">
      <Link href="/">Tic Tac Toe</Link>
    </div>
  </div>
</div>
