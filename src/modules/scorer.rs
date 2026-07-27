pub fn calculate_score(text: &str) -> i32 {

    let productive = vec![
        // College / WGU
        ("wgu", -20),
        ("western governors university", -20),
        ("course", -10),
        ("assignment", -15),
        ("lecture", -10),
        ("study", -10),
        ("exam", -10),
        ("quiz", -10),
        ("math", -10),
        ("algebra", -10),
        ("calculus", -10),
        ("statistics", -10),
        ("coding", -20),
        ("flashcards", -20),
        ("anki", -20),
        ("quizlet", -20),

        // CCNA / Networking
        ("ccna", -30),
        ("cisco", -30),
        ("packet tracer", -25),
        ("vlan", -20),
        ("trunking", -20),
        ("routing", -20),
        ("switching", -20),
        ("ospf", -20),
        ("bgp", -20),
        ("rip", -15),
        ("subnetting", -20),
        ("tcp", -15),
        ("ip address", -15),
        ("dns", -15),
        ("dhcp", -15),
        ("firewall", -15),
        ("wireshark", -20),
        ("infrastructure", -20),
        ("engineering", -20),
        ("hands-on", -20),
        ("lab", -30),
        ("labs", -30),    
        ("networking", -20),
        ("network", -20),
        ("networks", -20),

        // Geek Squad / ARA / IT
        ("geek squad", -20),
        ("best buy", -20),
        ("gsx", -15),
        ("repair", -15),
        ("troubleshooting", -20),
        ("diagnostics", -15),
        ("windows", -10),
        ("linux", -15),
        ("arch linux", -20),
        ("ssd", -10),
        ("nvme", -10),
        ("bios", -10),
        ("uefi", -10),
        ("firmware", -10),
        ("motherboard", -15),
        ("computer repair", -20),

        // Mindset
        ("christianity", -20),
        ("jesus", -20),
        ("christian", -20),
        ("god", -50),
        ("reality transurfing", -20),

        // Programming / AI
        ("rust", -20),
        ("python", -15),
        ("c++", -15),
        ("java", -15),
        ("programming", -15),
        ("coding", -15),
        ("github", -20),
        ("algorithm", -15),
        ("data structures", -15),
        ("machine learning", -20),
        ("artificial intelligence", -20),
        ("ai engineering", -20),
        ("notion", -20),
        ("github.com", -20),
        ("dynjee", -20),

        // Terminal / Linux workflow
        ("kitty", -20),
        ("terminal", -15),
        ("bash", -10),
        ("zsh", -10),
        ("shell", -10),
        ("command line", -10),
        ("ssh", -15),
        ("git", -15),
        ("docker", -15),
        ("pacman", -15),
        ("systemctl", -15),
        ("hyprctl", -15),
        // Electronics
        ("pcb", -20),
        ("esp32", -20),
        ("arduino", -15),
        ("raspberry pi", -20),
        ("soldering", -15),
        ("embedded systems", -20),
        ("diy", -20),
        ("electronics", -20),
        ("science", -20),
    ];



    let distracting = vec![
        // Algorithm / doom scrolling
        ("shorts", 40),
        ("youtube shorts", 40),
        ("tiktok", 40),
        ("instagram reels", 40),
        ("reels", 35),
        ("fyp", 35),
        ("for you page", 35),
        ("scrolling", 30),

        // Social media
        ("instagram", 30),
        ("facebook", 30),
        ("twitter", 25),
        ("x.com", 25),
        ("reddit", 25),
        ("linkedin", 10),

        // Memes / entertainment
        ("funny", 25),
        ("meme", 30),
        ("memes", 30),
        ("compilation", 25),
        ("fails", 25),
        ("prank", 25),
        ("reaction", 20),
        ("drama", 30),
        ("celebrity", 30),
        ("gossip", 35),
        ("tea", 25),

        // Specific rabbit holes
        ("emilyfan", 50),
        ("abt", 50),
        ("hormozi", 20),
        ("documentaries", 50),
        ("documentary", 50),

        // Romance / relationship content
        ("love", 25),
        ("dating", 30),
        ("relationship", 25),
        ("boyfriend", 30),
        ("girlfriend", 30),
        ("couple", 20),
        ("romance", 25),
        ("romantic", 25),
        ("crush", 25),
        ("heartbreak", 30),
        ("breakup", 30),
        ("toxic relationship", 35),
        ("relationship advice", 25),
        ("dating advice", 25),
        ("love story", 25),
        ("shipping", 20),
        ("thirst trap", 50),
        ("money", 20),
        ("compatibility", 20),

        // Astrology / prediction rabbit holes
        ("astrology", 25),
        ("zodiac", 25),
        ("horoscope", 30),
        ("birth chart", 30),
        ("tarot", 30),
        ("psychic", 30),
        ("fortune telling", 30),
        ("soulmate", 35),
        ("twin flame", 35),
        ("compatibility", 25),
        ("your sign", 25),
        ("moon sign", 25),
        ("venus sign", 25),
        ("mercury retrograde", 30),

        // General low-value content
        ("celebrity news", 35),
        ("influencer", 25),
        ("vlog", 15),
        ("reaction video", 25),
        ("brainrot", 25),
    ];


    let lower = text.to_lowercase();

    let mut score = 0;


    for (word, value) in productive {
        if lower.contains(word) {
            score += value;
        }
    }


    for (word, value) in distracting {
        if lower.contains(word) {
            score += value;
        }
    }


    score
}
