use leptos::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tab {
    Home,
    Operations,
    Contacts,
}

#[component]
fn App() -> impl IntoView {
    let (active_tab, set_active_tab) = create_signal(Tab::Home);

    view! {
        <div class="app-container">
            // Header Navigation
            <header class="header">
                <div class="logo">
                    <span class="logo-icon">{"⚡"}</span>
                    <span class="logo-text">{"APEX_CORE"}</span>
                </div>
                <nav class="nav-links">
                    <button 
                        class=move || if active_tab.get() == Tab::Home { "nav-btn active" } else { "nav-btn" }
                        on:click=move |_| set_active_tab.set(Tab::Home)
                    >
                        {"[ HOME ]"}
                    </button>
                    <button 
                        class=move || if active_tab.get() == Tab::Operations { "nav-btn active" } else { "nav-btn" }
                        on:click=move |_| set_active_tab.set(Tab::Operations)
                    >
                        {"[ OPERATIONS ]"}
                    </button>
                    <button 
                        class=move || if active_tab.get() == Tab::Contacts { "nav-btn active" } else { "nav-btn" }
                        on:click=move |_| set_active_tab.set(Tab::Contacts)
                    >
                        {"[ CONTACTS ]"}
                    </button>
                </nav>
            </header>

            // Main Content Area
            <main class="main-content">
                {move || match active_tab.get() {
                    Tab::Home => view! { <HomeView /> }.into_view(),
                    Tab::Operations => view! { <OperationsView /> }.into_view(),
                    Tab::Contacts => view! { <ContactsView /> }.into_view(),
                }}
            </main>

            // Footer
            <footer class="footer">
                <div class="footer-divider"></div>
                <div class="footer-content">
                    <span>{"STATUS: ALL SYSTEMS OPERATIONAL"}</span>
                    <span class="pulse-dot">{"🔴"}</span>
                    <span>{"© 2024 APEX_CORE INC."}</span>
                </div>
            </footer>
        </div>
    }
}

#[component]
fn HomeView() -> impl IntoView {
    let (reactor_power, set_reactor_power) = create_signal(50);
    
    let power_status = move || {
        let val = reactor_power.get();
        if val < 40 {
            ("UNDERPOWERED", "⚠️")
        } else if val < 85 {
            ("STABLE", "🟢")
        } else if val < 100 {
            ("OPTIMAL OVERCLOCK", "🔥")
        } else {
            ("CRITICAL OVERLOAD", "💥")
        }
    };

    view! {
        <div class="fade-in">
            <section class="hero-section">
                <h1 class="hero-title">
                    {"NEURAL-FLOW"} <br/>
                    <span class="text-red">{"OPERATIONS"}</span>
                </h1>
                <p class="hero-subtitle">
                    {"Next-generation cybernetic infrastructure for autonomous enterprises. We build, deploy, and secure high-frequency digital systems."}
                </p>
            </section>

            <div class="grid-2">
                // Interactive Widget: Reactor Control
                <div class="box interactive-box">
                    <div class="box-header">
                        <h3>{"CORE REACTOR SIMULATOR"}</h3>
                        <span class="box-tag">{"LIVE STATE"}</span>
                    </div>
                    <p class="box-desc">
                        {"Adjust the system core output to simulate load balancing across our global edge nodes."}
                    </p>
                    
                    <div class="reactor-display">
                        <span class="reactor-emoji">{move || power_status().1}</span>
                        <div class="reactor-metrics">
                            <span class="metric-label">{"OUTPUT LEVEL:"}</span>
                            <span class="metric-value">{move || reactor_power.get()}{"%"}</span>
                            <span class="metric-status">{move || power_status().0}</span>
                        </div>
                    </div>

                    <div class="slider-container">
                        <input 
                            type="range" 
                            min="10" 
                            max="110" 
                            value=move || reactor_power.get()
                            on:input=move |ev| {
                                if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                                    set_reactor_power.set(val);
                                }
                            }
                            class="red-slider"
                        />
                    </div>
                    <div class="btn-group">
                        <button class="action-btn" on:click=move |_| set_reactor_power.set(50)>{"RESET TO 50%"}</button>
                        <button class="action-btn red-btn" on:click=move |_| set_reactor_power.set(95)>{"MAX OVERCLOCK"}</button>
                    </div>
                </div>

                // Core Pillars
                <div class="box">
                    <div class="box-header">
                        <h3>{"SYSTEM ARCHITECTURE"}</h3>
                        <span class="box-tag">{"SPECS"}</span>
                    </div>
                    <div class="spec-list">
                        <div class="spec-item">
                            <span class="spec-icon">{"🛰️"}</span>
                            <div class="spec-text">
                                <strong>{"Zero-Latency Edge"}</strong>
                                <p>{"Global content delivery routing optimized via neural pathfinding."}</p>
                            </div>
                        </div>
                        <div class="spec-item">
                            <span class="spec-icon">{"🔐"}</span>
                            <div class="spec-text">
                                <strong>{"Quantum-Safe Cryptography"}</strong>
                                <p>{"Post-quantum encryption layers protecting every packet."}</p>
                            </div>
                        </div>
                        <div class="spec-item">
                            <span class="spec-icon">{"🤖"}</span>
                            <div class="spec-text">
                                <strong>{"Autonomous Recovery"}</strong>
                                <p>{"Self-healing cluster nodes that isolate and resolve failures."}</p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn OperationsView() -> impl IntoView {
    let (nodes, set_nodes) = create_signal(vec![
        "Node-Alpha (Tokyo)".to_string(),
        "Node-Beta (Zurich)".to_string(),
        "Node-Gamma (Austin)".to_string(),
    ]);
    
    let (new_node_name, set_new_node_name) = create_signal(String::new());

    let add_node = move |e: leptos::ev::SubmitEvent| {
        e.prevent_default();
        let name = new_node_name.get();
        if !name.trim().is_empty() {
            set_nodes.update(|n| n.push(format!("Node-{} (Virtual)", name.trim())));
            set_new_node_name.set(String::new());
        }
    };

    let remove_node = move |index: usize| {
        set_nodes.update(|n| {
            if n.len() > index {
                n.remove(index);
            }
        });
    };

    view! {
        <div class="fade-in">
            <section class="section-header">
                <h2 class="section-title">{"SYSTEM OPERATIONS"}</h2>
                <p class="section-subtitle">{"Deploy virtual nodes to our decentralized network mesh and monitor latency in real-time."}</p>
            </section>

            <div class="grid-3">
                <div class="box service-card">
                    <span class="card-emoji">{"📡"}</span>
                    <h4>{"TELEMETRY PIPELINES"}</h4>
                    <p>{"Real-time data ingestion pipelines capable of processing millions of events per second with sub-millisecond overhead."}</p>
                    <div class="card-footer">{"ACTIVE"}</div>
                </div>
                <div class="box service-card">
                    <span class="card-emoji">{"🧠"}</span>
                    <h4>{"COGNITIVE ROUTING"}</h4>
                    <p>{"Dynamic traffic shaping powered by predictive machine learning models to bypass regional internet blackouts."}</p>
                    <div class="card-footer">{"ACTIVE"}</div>
                </div>
                <div class="box service-card">
                    <span class="card-emoji">{"🛡️"}</span>
                    <h4>{"THREAT MITIGATION"}</h4>
                    <p>{"Automated DDoS shielding and malicious actor isolation running directly on our edge computing nodes."}</p>
                    <div class="card-footer">{"STANDBY"}</div>
                </div>
            </div>

            // Interactive Node Deployer
            <div class="box node-deployer-box">
                <div class="box-header">
                    <h3>{"ACTIVE NETWORK MESH"}</h3>
                    <span class="box-tag">{move || format!("{} NODES ONLINE", nodes.get().len())}</span>
                </div>

                <form on:submit=add_node class="
