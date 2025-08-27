project: Enforcer 
    what: 
    how: 
    why: 
    where: [test](files/enforcer_overview.md) 
    when: 
    who: Drew Brown 

    phase: Learn about Enforcer 
        what: 
        how: 
        why: 
        where: 
        when: 
        who: 

            task: Learn the Enforcer Standards 
                what: 
                how: 
                why: 
                where: [test](file://./files/enforcer_standards.md) 
                when: 
                who: 

            task: Learn the Enforcer Scope 
                what:  Linting, SAST, is DAST possible (benchmarking)? 
                how: hundreds of tools, offline only mode, single file, CLI MCP SaaS 
                why: simplicity/convenience, speed, security, stability, savings 
                where: 
                when: 
                who: 

            task: Learn the Enforcer architecture 
                what: 
                how: 
                why: 
                where: [enforcer architecture](file://./enforcer/enforcer_architecture.md) 
                when: 
                who: 

            task: Learn the Enforcer Parameters 
                what: 
                how: 
                why: 
                where: 
                when: 
                who: 

sarif

check parameters

tree-sitter

    phase: Implement TrainTrack 
        what: Add display_ , check_, etc to TrainTrack SAST 
        how: 
        why: 
        where: 
        when: 
        who: 

        task: Check that functions only have train input and output 
            what: 
            how: 
            why: 
            where: 
            when: 
            who: 

        task: Clone the codebase 
            what: 
            how: 
            why: 
            where: 
            when: 
            who: 





    phase: Startup uPlan 
        what: 
        how: 
        why: waterfall, uPlan can draft an entire business model based on a single word 
        where: [uPlan](files/uplan_product_management.md) 
        when: 
        who: 

        stage: Implement uPlan 
            what: 
            how: 
            why: 
            where: 
            when: 
            who: 

            task: Prompt fuzzy rules 
                what: fuzzy rules explain how to write the content of each prompt 
                how: 
                why: 
                where: 
                when: 
                who: 

            task: Prompt static rules 
                what: Static rules are 
                how: Use a uPlan SAST to return prompts based on uPlan rules 
                why: 
                where: 
                when: 
                who: 

    phase: Startup Cloner 
        what: Cloner is a company that mimics stateless input/output software 
        how: Code is installed, tested, refactored, and re-implemented in Rust 
        why: Clones are more secure, stable, simple, speedy, and offer savings 
        where: [Cloner Overview](cloner/cloner_overview.md) 
        when: 
        who: ape_junior_llm 

        stage: Research Cloner 
            what: Research searches for and aggregates data for a business plan 
            how: Prompt a deep research LLM to complete the tasks in this stage 
            why: The data helps the LLM develop the Enforcer software suite 
            where: [Cloner Research](cloner/cloner_research.md) 
            when: 
            who: ape_research_llm 

            task: Produce the Profile Page 
                what: The Profile Page is the executive summary and canvas 
                how: Edit the text to be concise and in the markdown format 
                why: The summary helps potential stakeholders see the plan 
                where: [Profile Page](cloner/profile_page.md) 
                when: 
                who: ape_research_llm 

            task: Produce the Problem Page 
                what: The Problem Page is a brief list of the pain points 
                how: Research the persona pain points and find statistics 
                why: The problem being solved must be defined & supported 
                where: [Problem Page](file://./templates/problem_page.tt) 
                when: 
                who: ape_research_llm 

            task: Conduct a CCA 
                what: A comprehensive competitor analysis evaluates competitors 
                how: List, research, copy competitor collateral, and analyze it 
                why: Analyzing the competitors helps define the growth strategy 
                where: [CCA template](file://./templates/comprehensive_competitor_analysis.tt) 
                when: 
                who: ape_research_llm 

            task: Produce the Partner Page 
                what: The Partner Page lists the entities are Founders 
                how: Edit the notes to share the background of partners 
                why: The company leadership needs to be defined for all 
                where: [Partner Page](file://./templates/partner_page.tt) 
                when: 
                who: ape_research_llm 

            task: Produce the Program Page 
                what: The Program Page lists the software tools included 
                how: Research and list all the related tools and plugins 
                why: The scope of the program is defined using a Promise 
                where: [Program Page](file://./templates/program_page.tt) 
                when: 
                who: ape_research_llm 

            task: Produce the Profess Page 
                what: The Profess Page summarizes relevant academic research papers 
                how: Find and download research papers and identify relevant ideas 
                why: Research papers help support design decisions and finds trends 
                where: [Profess Page](file://./templates/profess_page.tt) 
                when: 
                who: 

            task: Produce the Process Page 
                what: The Process Page describes the standard operating procedures 
                how: Edit the notes describing the processes so all can understand 
                why: Everyone needs to have a clear understanding of the processes 
                where: [Process Page](file://./templates/process_page.tt) 
                when: 
                who: ape_research_llm 

            task: Produce the Product Page 
                what: The Product Page describes the attibutes and features 
                how: Edit the notes to describe the product very concisely 
                why: The description can narrow down the product benefits 
                where: [Product Page](file://./templates/product_page.tt) 
                when: 
                who: ape_research_llm 

            task: Produce the Promise Page 
                what: The Promise Page defines the solution and expected results 
                how: Search through the documentation in this uPlan and draft it 
                why: It is critical to describe why this solution is needed now 
                where: [Promise Page](file://./templates/promise_page.tt) 
                when: 
                who: ape_research_llm 

            task: Produce the Persona Page 
                what: The Persona Page describes the potential customers 
                how: Research the problem and describe those that suffer 
                why: The persona is used to help find potential customers 
                where: [Persona Page](file://./templates/persona_page.tt) 
                when: 
                who: ape_research_llm 

            task: Produce the Passage Page 
                what: The Passage Page illustrates the overview of the market 
                how: Describe the market in terms of TAM, SAM, and SOM metrics 
                why: The market size helps define the size of the opportunity 
                where: [Passage Page](file://./templates/passage_page.tt) 
                when: 
                who: ape_research_llm 

            task: Produce the Protect Page 
                what: The Protect Page describes the "moat" against competitors 
                how: Research and clearly define the way to defend the business 
                why: The business should be hard for competitors to copy easily 
                where: [Protect Page](file://./templates/protect_page.tt) 
                when: 
                who: ape_research_llm 

            task: Produce the Pricing Page 
                what: The Pricing Page sets the pricing for our services 
                how: Research competitor prices and set relative pricing 
                why: Setting prices helps stop Founders from mispricing 
                where: [Pricing Page](file://./templates/pricing_page.tt) 
                when: 
                who: ape_research_llm 

            task: Produce the Protest Page 
                what: The Protest Page lists reasons not to procede 
                how: Use a deep research model to dispute assertions 
                why: Protesting the model improves risk mitigation 
                where: [Protest Page](file://./templates/protest_page.tt) 
                when: 
                who: ape_research_llm 

            task: Produce the Patrons Page 
                what: The Patrons Page has the current customers (or waitlist) 
                how: Edit the information related to patrons that are listed 
                why: Showing there are patrons helps show product market fit 
                where: [Patrons Page](file://./templates/patrons_page.tt) 
                when: 
                who: ape_research_llm 

            task: Produce the Promote Page 
                what: The Promote Page shows how the service will be marketed 
                how: Edit the notes to share the marketing strategy and status 
                why: The (go-to) marketing strategy shows how we plan to grow 
                where: [Promote Page](file://./templates/promote_page.tt) 
                when: 
                who: ape_research_llm 

            task: Produce the Propose Page 
                what: The Propose Page states a disputed value proposition 
                how: Draft a sales script based on common patron rebuttals 
                why: The script is used to prepare for handling objections 
                where: [Propose Page](file://./templates/propose_page.tt) 
                when: 
                who: ape_research_llm 

            task: Produce Pitcher Page 
                what: The Pitcher Page is the pitch deck used for presentations 
                how: Create a concise pitch deck that shares the company story 
                why: A pitch deck helps the Founders get investors and partners 
                where: [Pitcher Page](file://./templates/pitcher_page.tt) 
                when: 
                who: ape_research_llm 

            task: Produce the Procede Page 
                what: The Procede Page is a call to action that ends a pitch 
                how: Edit the notes to create an entertaining call to action 
                why: Potential stakeholders need to clearly see what we want 
                where: [Procede Page](file://./templates/procede_page.tt) 
                when: 
                who: ape_research_llm 

        stage: Sell Cloner services 
            what: 
            how: 
            why: 
            where: 
            when: 
            who: 

    phase: Startup BugHunter 
        what: 
        how: 
        why: 
        where: https://enigma-agent.com 
        when: 
        who: 

    phase: Startup ModuleManual 
        what: 
        how: 
        why: (Affiliate) marketing 
        where: 
        when: 
        who: 

        stage: Develop Instabot 
            what: 
            how: 
            why: Plixi Affiliate Marketing 
            where: 
            when: 
            who: 

    phase: Startup reCover 
        what: reCover specializes in digital disaster recovery 
        how: 
        why: 
        where: 
        when: 
        who: 

    phase: Startup Auditor 
        what: 
        how: 
        why: 
        where: 
        when: 
        who: 

        stage: Tool 1 
            what: 
            how: 
            why: 
            where: 
            when: 
            who: 

    phase: Startup MCPammer 
        what: 
        how: 
        why: 
        where: 
        when: 
        who: 

    phase: Startup Compliance 
        what: 
        how: Like Vanta, but only in cod Vanta supports 35+ leading security and privacy frameworks 
        why: 
        where: 
        when: 
        who: 

    phase: Startup DevMentor 
        what: 
        how: 
        why: polyglot 
        where: 
        when: 
        who: 

    phase: Startup PortAuthority 
        what: 
        how: https://www.codeflash.ai 
        why: 
        where: 
        when: 
        who: 

    phase: Startup AppSec 
        what: 
        how: 
        why: 
        where: 
        when: 
        who: 

    phase: Startup Reporter 
        what: 
        how: Threaten to publish scans 
        why: 
        where: 
        when: 
        who: 

    phase: Startup ServiceLater 
        what: 
        how: 
        why: 
        where: 
        when: 
        who: 

    phase: Startup Integrator 
        what: Integrate APIs like Zapier 
        how: AliasAPI, combining codebases / repos 
        why: 
        where: 
        when: 
        who: 

    phase: Startup MidManager 
        what: 
        how: Manage freelance software devs 
        why: 
        where: 
        when: 
        who: 

    phase: Startup HighQ 
        what: Refactor code to be high quality 
        how: Auto Debug, adding tests and running them 
        why: 
        where: 
        when: 
        who: 

    phase: Startup PullR 
        what: Pull requests 
        how: fix software and do pull requests, offer to maintain open source software 
        why: 
        where: 
        when: 
        who: 

    phase: Startup Competitor 
        what: 
        how: Solicit competitor clients , Vanta, Snyk, SonarQube 
        why: 
        where: 
        when: 
        who: 

    phase: Startup CItool 
        what: 
        how: GitHub and Bitbucket 
        why: 
        where: 
        when: 
        who: 

    phase: Startup Penetrator 
        what: 
        how: 
        why: 
        where: 
        when: 
        who: 

    phase: Startup Jankins 
        what: Update Jenkins plugins or replace Jenkins altogether 
        how: 
        why: 
        where: 
        when: 
        who: 

    phase: Startup Polyglot_Perfectionist 
        what: 
        how: 
        why: Developer speed, upskilling cheaper labor, equity partner 
        where: 
        when: 
        who: 

    phase: Startup BorgBot 
        what: 
        how: 
        why: 
        where: 
        when: 
        who: 

    phase: Startup Washington 
        what: 
        how: 
        why: 
        where: https://www.linkedin.com/in/crystal-washington-cyber/details/experience/ 
        when: 
        who: 

    phase: Startup LLMTrainer 
        what: LLMTrainer (to teach LLMs new languages) 
        how: Re-enforcement learning (get the quote from OpenAI) 
        why: 
        where: https://www.youtube.com/watch?v=bNEvJYzoa8A 
        when: 
        who: 

    phase: Startup InterviewBot 
        what: 
        how: 
        why: 
        where: 
        when: 
        who: 

    phase: Startup SExpert 
        what: 
        how: Use AI to write a book about AppSec et al using all the above research 
        why: Security Expert 
        where: 
        when: 
        who: 

    phase: Startup NeuroCode 
        what: 
        how: 
        why: Accomidation for neurodivergent developers 
        where: 
        when: 
        who: 

    phase: Startup FinIsh 
        what: fintech proptech 
        how: 
        why: 
        where: 
        when: 
        who: 

    phase: Startup Fine 
        what: Fine-tune a model to write very high quality code 
        how: Reinforcemet learning TrainTrack / custom codebase 
        why: A ServiceNow model for your codebase or TrainTrack 
        where: 
        when: 
        who: 

    phase: Startup HackThong 
        what: 
        how: Enter Hack-a-thons for prize money 
        why: 
        where: 
        when: 
        who: 

    phase: Startup PitchBitch 
        what: PitchBitch is legal startup corporate espionage 
        how: Record startup pitches, uPlan, and go to market 
        why: To be able to say, "Ain't that about a bitch?!" 
        where: 
        when: 
        who: 

    phase: Test Enforcer 
        what: 
        how: 
        why: 
        where: https://github.com/LEGION-Sec/SAST-Benchmark-Suite 
        when: 
        who: 

    phase: Startup You 
        what: Use personality profiling to emulate customers 
        how: MLM 
        why: 
        where: 
        when: 
        who: 

    phase: Startup Brokerage 
        what: 
        how: Scrape and solicit Renovators to borrow 
        why: 
        where: 
        when: 
        who: 

    phase: Startup StepsTEAM 
        what: 
        how: 
        why: 
        where: 
        when: 
        who: 

    phase: Startup FinanceFlips 
        what: 
        how: 
        why: 
        where: 
        when: 
        who: 
