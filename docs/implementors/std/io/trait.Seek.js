(function() {var implementors = {};
implementors["buf_redux"] = ["impl&lt;R:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Seek.html\" title=\"trait std::io::Seek\">Seek</a>, Rs:&nbsp;<a class=\"trait\" href=\"buf_redux/strategy/trait.ReadStrategy.html\" title=\"trait buf_redux::strategy::ReadStrategy\">ReadStrategy</a>, Ms:&nbsp;<a class=\"trait\" href=\"buf_redux/strategy/trait.MoveStrategy.html\" title=\"trait buf_redux::strategy::MoveStrategy\">MoveStrategy</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Seek.html\" title=\"trait std::io::Seek\">Seek</a> for <a class=\"struct\" href=\"buf_redux/struct.BufReader.html\" title=\"struct buf_redux::BufReader\">BufReader</a>&lt;R, Rs, Ms&gt;","impl&lt;W:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Seek.html\" title=\"trait std::io::Seek\">Seek</a>, Fs:&nbsp;<a class=\"trait\" href=\"buf_redux/strategy/trait.FlushStrategy.html\" title=\"trait buf_redux::strategy::FlushStrategy\">FlushStrategy</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Seek.html\" title=\"trait std::io::Seek\">Seek</a> for <a class=\"struct\" href=\"buf_redux/struct.BufWriter.html\" title=\"struct buf_redux::BufWriter\">BufWriter</a>&lt;W, Fs&gt;",];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
