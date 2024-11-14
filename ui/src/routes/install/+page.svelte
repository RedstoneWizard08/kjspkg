<script lang="ts">
    import "highlight.js/styles/github-dark.css";
    import hljs from "highlight.js/lib/core";
    import bash from "highlight.js/lib/languages/bash";
    import pwsh from "highlight.js/lib/languages/powershell";
    import { _ } from "svelte-i18n";
    import { CodeBlock, storeHighlightJs, Tab, TabGroup } from "@skeletonlabs/skeleton";
    import { fly } from "svelte/transition";

    hljs.registerLanguage("bash", bash);
    hljs.registerLanguage("powershell", pwsh);

    $storeHighlightJs = hljs;

    let group = $state(0);

    const linuxMacScript =
        "curl -fsSL https://github.com/RedstoneWizard08/kjspkg/raw/main/install_cli.sh | bash";

    const binstallScript = "cargo binstall kjspkg";

    const cargoScript = "cargo install kjspkg";

    const sourceLinuxMac = `# For Linux/MacOS
git clone https://github.com/RedstoneWizard08/kjspkg
cd kjspkg
cargo build --release -p kjspkg
cp target/release/kjspkg /path/to/your/bin/dir/kjspkg`;

    const sourceWindows = `# For Windows
git clone https://github.com/RedstoneWizard08/kjspkg
cd kjspkg
cargo build --release -p kjspkg
cp target/release/kjspkg.exe C:\\path\\to\\your\\bin\\dir\\kjspkg.exe`;
</script>

<svelte:head>
    <title>{$_("install.header")} - KJSPKG</title>
</svelte:head>

<div class="card rounded-xl p-4" in:fly={{ y: 20 }}>
    <p class="mb-4 text-2xl font-bold">{$_("install.header")}</p>

    <TabGroup>
        <Tab bind:group name={$_("install.script_linux_mac")} value={0}
            >{$_("install.script_linux_mac")}</Tab
        >
        <Tab bind:group name={$_("install.script_win")} value={1}>{$_("install.script_win")}</Tab>
        <Tab bind:group name={$_("install.binaries")} value={3}>{$_("install.binaries")}</Tab>
        <Tab bind:group name={$_("install.binstall")} value={2}>{$_("install.binstall")}</Tab>
        <Tab bind:group name={$_("install.cargo")} value={4}>{$_("install.cargo")}</Tab>
        <Tab bind:group name={$_("install.source")} value={5}>{$_("install.source")}</Tab>

        <svelte:fragment slot="panel">
            <!-- I could have very much simplified the `in:fly` stuff using a div, but I don't feel like doing that now. -->
            {#if group == 0}
                <p class="mb-4" in:fly={{ y: 20 }}>
                    {$_("install.script_linux_mac.pre")}
                </p>

                <div in:fly={{ y: 20 }}>
                    <CodeBlock language="bash" code={linuxMacScript} lineNumbers />
                </div>

                <p class="mt-4" in:fly={{ y: 20 }}>
                    {$_("install.script_linux_mac.post.1")}
                    <code>~/.local/bin</code>
                    {$_("install.script_linux_mac.post.2")}
                    <code>PATH</code>,
                    {$_("install.script_linux_mac.post.3")}
                </p>
            {:else if group == 1}
                <p in:fly={{ y: 20 }}>{$_("install.coming_soon")}</p>
            {:else if group == 2}
                <p class="mb-4" in:fly={{ y: 20 }}>
                    {$_("install.binstall.pre.1")}
                    <code>cargo-binstall</code>,
                    {$_("install.binstall.pre.2")}
                </p>

                <div in:fly={{ y: 20 }}>
                    <CodeBlock language="bash" code={binstallScript} lineNumbers />
                </div>

                <p class="mt-4" in:fly={{ y: 20 }}>
                    {$_("install.binstall.post.1")}
                    <code>~/.cargo/bin</code>
                    {$_("install.binstall.post.2")}
                    <code>PATH</code>,
                    {$_("install.binstall.post.3")}
                </p>
            {:else if group == 3}
                <p>
                    {$_("install.binaries.pre.1")}
                    <a
                        href="https://github.com/RedstoneWizard08/kjspkg/releases/latest"
                        class="anchor">{$_("install.binaries.pre.anchor")}</a
                    >
                    {$_("install.binaries.pre.2")}
                </p>
                <p>
                    {$_("install.binaries.post.1")}
                    <a href="https://github.com/RedstoneWizard08/kjspkg/releases" class="anchor"
                        >{$_("install.binaries.post.anchor")}</a
                    >.
                </p>
            {:else if group == 4}
                <p class="mb-4" in:fly={{ y: 20 }}>
                    {$_("install.cargo.pre.1")}
                    <code>cargo install</code>,
                    {$_("install.cargo.pre.2")}
                    <a href="https://crates.io/crates/kjspkg" class="anchor"
                        >{$_("install.cargo.crates_io")}</a
                    >,
                    {$_("install.cargo.pre.3")}
                </p>

                <div in:fly={{ y: 20 }}>
                    <CodeBlock language="bash" code={cargoScript} lineNumbers />
                </div>

                <p class="mt-4" in:fly={{ y: 20 }}>
                    {$_("install.cargo.post.1")}
                    <code>~/.cargo/bin</code>
                    {$_("install.cargo.post.2")}
                    <code>PATH</code>,
                    {$_("install.cargo.post.3")}
                </p>

                <p in:fly={{ y: 20 }}>
                    {$_("install.cargo.note.1")}
                    <a href="https://rustup.rs" class="anchor">{$_("install.cargo.rust")}</a>
                    {$_("install.cargo.note.2")}
                </p>
            {:else if group == 5}
                <p class="mb-4" in:fly={{ y: 20 }}>
                    {$_("install.source.pre")}
                </p>

                <div in:fly={{ y: 20 }}>
                    <CodeBlock language="bash" code={sourceLinuxMac} lineNumbers />
                    <CodeBlock language="powershell" code={sourceWindows} lineNumbers />
                </div>

                <p class="mt-4" in:fly={{ y: 20 }}>
                    {$_("install.source.post")}
                </p>
            {/if}
        </svelte:fragment>
    </TabGroup>
</div>
