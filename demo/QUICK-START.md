# omega-synesthesia Demo - Quick Start Guide

## 🎯 Choose Your Demo Option

### 1️⃣ Web Demo (Easiest - 5 minutes)

**Best for:** Third-party reviewers, quick demos, sharing online

```bash
cd demo/web-viewer
npm install
npm run dev
```

Open `http://localhost:5173` in your browser.

✅ No Rust required
✅ Works on any platform
✅ Easy to share (deploy to Netlify)
✅ Full interactive controls

---

### 2️⃣ Native Rust Demo (Best Performance)

**Best for:** Technical validation, performance testing

```bash
cd omega
cargo run --example week3_final_integration --release
```

✅ 2.79ms latency (19.7x faster!)
✅ Full GPU acceleration
✅ Professional rendering quality

---

### 3️⃣ Interactive Demo (For Presentations)

**Best for:** Live demonstrations, testing controls

```bash
cd omega
cargo run --example interactive_demo --release
```

**Controls:**
- SPACE - Play/Pause
- 1-4 - Camera modes
- G - Change genre
- +/- - Speed
- R - Reset
- H - Help

---

## 📚 Full Documentation

- **Web Demo:** `demo/web-viewer/README.md`
- **Demo Guide:** `demo/DEMO-GUIDE.md`
- **Validation Guide:** `demo/THIRD-PARTY-VALIDATION-GUIDE.md`
- **Release Notes:** `docs/RELEASE-NOTES-V1.0.0.md`

---

## 🚀 Quick Deploy (Share with Anyone)

### Deploy Web Demo to Netlify:

```bash
cd demo/web-viewer
npm run build

# Upload dist/ folder to Netlify
# Or: npx netlify deploy --prod --dir=dist
```

**Result:** Anyone can test at your-url.netlify.app!

---

## ✅ Validation Checklist

- [ ] Run one demo successfully
- [ ] Test genre switching
- [ ] Test camera modes
- [ ] Check performance metrics
- [ ] Review documentation

---

**Need Help?** See `demo/THIRD-PARTY-VALIDATION-GUIDE.md` for complete details.
