# 7. Simpler Showcase Dashboard - Foundational Trading Room

## 🎯 Purpose & Functionality
The Simpler Showcase Dashboard provides **entry-level members** with access to foundational trading education, basic trading room access, and beginner-friendly content. This dashboard is designed for **new traders** seeking to learn the basics of trading and build a strong foundation in market analysis.

## 📋 Population Logic
- **Called On:** Navigation to `/dashboard/simpler-showcase/` URL
- **Populated With:** Basic membership content, showcase trading room, foundational education
- **Data Source:** WooCommerce Membership ID: 'simpler_showcase' + custom post content
- **Trigger:** Click on navigation link or direct URL access
- **Refreshes:** New showcase content, trading room schedule updates, membership status

## 🏗️ Content Components
- **Breakout Room Access** (JWT authenticated)
- **Foundational Video Content**
- **Basic Trading Education**
- **Beginner Strategy Guides**
- **Market Basics Tutorials**
- **Entry-Level Resources**
- **Community Support Access**

## 💻 Complete Implementation Code

See original Dashboards.md lines 1383-1995 for full PHP implementation with:
- Learning progress tracking
- Educational path stages (4 stages: Basics, Chart Analysis, Strategy Development, Advanced Concepts)
- Video categories (Basics, Analysis, Strategies, Psychology)
- Beginner strategies with difficulty ratings
- Interactive tutorials
- Breakout room integration with JWT authentication

**Key Features:**
- ✅ Progressive learning path with locked/unlocked stages
- ✅ Educational videos organized by category
- ✅ Beginner-friendly strategies
- ✅ Interactive tutorials
- ✅ Breakout room access
- ✅ Learning progress dashboard

**Membership ID:** `simpler_showcase`  
**Access Level:** Foundational  
**Trading Room:** Breakout Room with JWT authentication
